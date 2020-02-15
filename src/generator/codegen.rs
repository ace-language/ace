use cranelift::prelude::*;
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use std::mem;


pub struct Compiler {
    module: Module<SimpleJITBackend>,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            module: Module::new(SimpleJITBuilder::new(default_libcall_names())),
        }
    }

    fn create_func(&mut self, name: &str) {
        let mut ctx = self.module.make_context();
        let mut func_ctx = FunctionBuilderContext::new();

        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(types::I32));
        sig.returns.push(AbiParam::new(types::I32));

        let func = self.module
        .declare_function(name, Linkage::Local, &sig)
        .unwrap();

        // ctx.func.signature = sig;
        // ctx.func.name = ExternalName::user(0, func.as_u32());

        {
            let mut bcx = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
            let block = bcx.create_block();
            bcx.switch_to_block(block);
            bcx.append_block_params_for_function_params(block);

            // function logic itself
            let param = bcx.block_params(block)[0];
            let cst = bcx.ins().iconst(types::I32, 37);
            let add = bcx.ins().iadd(cst, param);
            bcx.ins().return_(&[add]);

            bcx.seal_all_blocks();
            bcx.finalize();
        }
    }
}