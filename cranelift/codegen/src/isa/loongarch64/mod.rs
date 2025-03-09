// Loongarch 64-bit ISA

use core::fmt;
use std::string::String;
use std::vec::Vec;

use target_lexicon::Triple;

use crate::ir;
use crate::isa::{Builder as IsaBuilder, TargetIsa};
use crate::machinst::Reg;
use crate::settings::{self as shared_settings};

mod abi;
mod settings;

struct Loongarch64Backend {
    triple: Triple,
    flags: shared_settings::Flags,
    isa_flags: loongarch64_settings::Flags,
}

impl TargetIsa for Loongarch64Backend {
    fn name(&self) -> &'static str {
        "loongarch64"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn flags(&self) -> &shared_settings::Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.isa_flags.iter().collect()
    }

    fn dynamic_vector_bytes(&self, dynamic_ty: crate::ir::Type) -> u32 {
        // LASX has 256-bit vectors
        32
    }

    fn compile_function(
            &self,
            func: &crate::ir::Function,
            domtree: &crate::dominator_tree::DominatorTree,
            want_disasm: bool,
            ctrl_plane: &mut cranelift_control::ControlPlane,
        ) -> crate::CodegenResult<crate::machinst::CompiledCodeStencil> {
        unimplemented!()
    }

    #[cfg(feature = "unwind")]
    fn map_regalloc_reg_to_dwarf(
            &self,
            _: crate::machinst::Reg,
        ) -> Result<u16, super::unwind::systemv::RegisterMappingError> {
        unimplemented!()
    }

    #[cfg(feature = "unwind")]
    fn emit_unwind_info(
            &self,
            _: &crate::machinst::CompiledCode,
            _: crate::isa::unwind::UnwindInfoKind,
        ) -> crate::CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        unimplemented!()
    }

    #[cfg(feature = "unwind")]
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
        unimplemented!()
    }

    fn text_section_builder(&self, num_labeled_funcs: usize) -> std::prelude::v1::Box<dyn crate::TextSectionBuilder> {
        unimplemented!()
    }

    fn function_alignment(&self) -> super::FunctionAlignment {
        unimplemented!()
    }

    fn page_size_align_log2(&self) -> u8 {
        // Support 4k page, also support 16k page
        12
    }

    #[cfg(feature = "disas")]
    fn to_capstone(&self) -> Result<capstone::Capstone, capstone::Error> {
        // Wait the release of capstone 6.0.0 to support loongarch64
        Err(capstone::Error::UnsupportedArch)
    }

    fn pretty_print_reg(&self, reg: Reg, size: u8) -> String {
        // TODO
        format!("{:#?}", reg)
    }

    fn has_native_fma(&self) -> bool {
        // Useless on loongarch64, just return true
        true
    }

    fn has_x86_blendv_lowering(&self, ty: crate::ir::Type) -> bool {
        false
    }

    fn has_x86_pmaddubsw_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmulhrsw_lowering(&self) -> bool {
        false
    }

    fn has_x86_pshufb_lowering(&self) -> bool {
        false
    }

    fn default_argument_extension(&self) -> crate::ir::ArgumentExtension {
        ir::ArgumentExtension::Sext
    }
}

impl fmt::Display for Loongarch64Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Loongarch64Backend")
            .field("triple", &self.triple)
            .field("flags", &self.flags)
            .field("isa_flags", &self.isa_flags)
            .finish()
    }
}
