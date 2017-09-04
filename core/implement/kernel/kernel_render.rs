extern crate gl;

use ::std::str;
use ::std::ops::DerefMut;
use std::collections::HashMap;
use std::cell::RefCell;   

use interface::i_ele;
use interface::i_window::IWindow;
use interface::i_renderobj::RenderDevice;
use interface::i_renderobj;
use interface::i_renderpass;
use interface::i_component;

use implement::window::winglutin::WinGlutin;
use implement::capability::capability_gl;
use implement::render::util_gl;
use implement::render::shader_collection;
use implement::render::texture_collection;
use implement::render::router;
use implement::render::renderdevice_gl;
// use implement::render::renderpass_default;


// pub struct RendererWrap {
//     pub _renderer: RefCell< Renderer >,
// }

// impl RendererWrap {
//     pub fn init() -> RendererWrap {
//         RendererWrap {
//             _renderer: RefCell::new( Renderer::init().unwrap() ),
//         }
//     }
// }



pub struct Renderer {
    _win: WinGlutin,
    _rp: Vec< Box< i_renderpass::IRenderPass > >,
    _map_string_to_rp: HashMap< String, usize >,
    pub _shader_collection: RefCell< shader_collection::ShaderCollection >,
    _texture_collection: texture_collection::TextureCollection,
    _shader_programs: Vec< u64 >,
    _draw_groups: RefCell< Vec< renderdevice_gl::RenderDrawGroup > >,
    _vaos: Vec< gl::types::GLuint >,
    _vbos: Vec< gl::types::GLuint >,
    _objs: RefCell< Vec< Box< i_ele::Ele > > >,
    _uniforms: RefCell< renderdevice_gl::RenderUniformCollection >,
    _draw_group_uniforms: RefCell< Vec< Vec< u64 > > >,
    _shaders_compiled: Vec< gl::types::GLuint >,
    //todo: to be removed
    _current_shader_program: u64,
}

impl Drop for Renderer {
    fn drop( & mut self ) {
        unsafe {
            gl::DeleteBuffers( self._vaos.len() as _, self._vaos.as_ptr(), );

            gl::DeleteBuffers( self._vbos.len() as _, self._vbos.as_ptr(), );
        }
        self._shader_collection.borrow_mut().clear().is_ok(); //does DeleteProgram
        unsafe {
            for i in self._shaders_compiled.iter() {
                gl::DeleteShader( *i );
            }
        }
    }
}


impl Renderer {
    pub fn init() -> Result< Renderer, & 'static str > {
        let rk = Renderer {
            _win: IWindow::init( 500, 500 ),
            _rp: vec![],
            _map_string_to_rp: HashMap::new(),
            _shader_collection: RefCell::new( Default::default() ),
            _texture_collection: Default::default(),
            _shader_programs: vec![],
            _draw_groups: RefCell::new( vec![] ),
            _vaos: vec![],
            _vbos: vec![],
            _objs: RefCell::new( vec![] ),
            _uniforms: RefCell::new( Default::default() ),
            _draw_group_uniforms: RefCell::new( vec![] ),
            _shaders_compiled: vec![],
            _current_shader_program: 0,
        };
        match rk._win.make_current() {
            Err( e ) => return Err( e ),
            _ => (),
        }
        let cap = capability_gl::query_gl();
        println!( "{}", cap );
        Ok( rk )
    }
    pub fn load_shader( & mut self, sources: &[ ( &str, util_gl::ShaderType ) ] ) -> Result< ( u64 ), & 'static str > {
        let mut compiled_shaders = vec![];
        for &(ref src, ref src_type ) in sources.into_iter() {
            let s = match util_gl::load_and_compile_shader( src, *src_type ) {
                Ok( o ) => o,
                Err( o ) => {
                    println!( "{}", o );
                    return Err( "error loading shader" )
                }
            };
            util_gl::check_last_op();
            compiled_shaders.push( s );
        }
        {
            let i = self._shader_programs.len();
            {
                self._shader_collection.borrow_mut().put( i as u64, router::ShaderType::GLSL, util_gl::create_program_from_shaders( compiled_shaders.as_slice() ) as _, String::from("ads_program") ).is_ok();
            }
            let shader_program = self._shader_collection.borrow_mut().get( i as u64 ).unwrap();
            unsafe {
                gl::UseProgram( shader_program as _ );
                self._current_shader_program = i as u64;
                gl::BindFramebuffer( gl::FRAMEBUFFER, 0 );
                gl::Enable( gl::DEPTH_TEST );
            }
            self._shader_programs.push( shader_program as u64 );
            {
                self._shaders_compiled.append( & mut compiled_shaders );
            }
            Ok( i as u64 )
        }
    }
    pub fn load_texture( & mut self, description: String, image: &[u8], w: usize, h: usize ) -> Result< ( u64 ), & 'static str > {
        let shader_program_internal = self._shader_collection.borrow_mut().get( self._current_shader_program ).unwrap();
        let handle = match util_gl::load_texture( shader_program_internal as _, 0, image, w, h ) {
            Ok( h ) => h,
            _ => return Err( "loading texture failed" )
        };
        let h = match self._texture_collection.add( router::ShaderType::GLSL, handle as _, description ) {
            Ok( h ) => h,
            Err( e ) => return Err( e ),
        };
        Ok( h )
    }
    pub fn create_draw_group( & mut self, prim_type: i_renderobj::RenderObjType ) -> Result< ( gl::types::GLuint, gl::types::GLuint, usize ), & 'static str > {
        let mut obj_vao = 0;
        let mut obj_vbo = 0;
        unsafe {
            gl::GenVertexArrays( 1, & mut obj_vao );
            util_gl::check_last_op();
            gl::GenBuffers( 1, & mut obj_vbo );
            util_gl::check_last_op();
        }
        
        let draw_group = match prim_type {
            i_renderobj::RenderObjType::TRI => renderdevice_gl::RenderDrawGroup::init_with_default_format_triangle( obj_vao as _, obj_vbo as _ ),
            i_renderobj::RenderObjType::POINT => renderdevice_gl::RenderDrawGroup::init_with_default_format_point( obj_vao as _, obj_vbo as _ ),
            _ => return Err("unsupported primitive type for draw group detected")
        };
        // let mut draw_group = renderdevice_gl::RenderDrawGroup::init_with_default_format( obj_vao as _, obj_vbo as _ );
        self._draw_groups.borrow_mut().push( draw_group );
        Ok( ( obj_vao, obj_vbo, self._draw_groups.borrow_mut().len() - 1) )
    }
    #[allow(unused)]
    pub fn add_obj( renderer: & mut Renderer, name: &str, e: i_ele::Ele ) -> Result< ( usize ), & 'static str > {

        let index = renderer._objs.borrow_mut().len();
        renderer._objs.borrow_mut().push( Box::new( e ) );

        //load component data
        match renderer._objs.borrow_mut()[index].update_components_from_impl() {
            Err( e ) => { return Err( e ) },
            _ => (),
        }

        //detect command to flush and process all data in buffer
        let mut trigger_to_process_objs = false;
        let mut group_id = 0;
        for j in renderer._objs.borrow_mut()[index]._components.iter() {
            match j.as_any().downcast_ref::< i_component::ComponentDrawGroupDispatch >() {
                Some( o ) => {
                    println!("detected trigger for draw group dispatch");
                    trigger_to_process_objs = true;
                    group_id = o._group_id;
                },
                None => {},
            }
        }

        if trigger_to_process_objs {
            Renderer::process_objs( renderer, group_id )?
        }

        Ok( renderer._objs.borrow_mut().len() )  
    }
    pub fn process_objs( renderer: & mut Renderer, group_index: usize ) -> Result< (), & 'static str > {       
        println!("objects size: {}", renderer._objs.borrow_mut().len() );
        
        for i in renderer._objs.borrow_mut().iter() {

            for j in i._components.iter() {

                //downcasting: https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
                match j.as_any().downcast_ref::< i_component::ComponentRenderBuffer >() {
                    Some( o ) => {
                        println!("buffer flushed");
                        match o.flush_into_render_device( & mut renderer._draw_groups.get_mut()[ group_index ] ) {
                            Err( e ) => return Err( e ),
                            _ => { continue; },
                        }
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentRenderUniform >() {
                    Some( o ) => {
                        println!("uniform flushed");
                        let shader_program = renderer._shader_collection.borrow_mut().get( renderer._current_shader_program ).unwrap();
                        match o.flush_into_uniform_collection( shader_program, & mut renderer._uniforms.borrow_mut() ) {
                            Err( e ) => return Err( e ),
                            _ => { continue; },
                        }
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupClear >() {
                    Some( o ) => {
                        println!("draw group clear");
                        renderer.reset_draw_group_data( &[ o._group_id ] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupBind >() {
                    Some( o ) => {
                        println!("draw group bind");
                        renderer.bind_draw_group_data( &[ o._group_id ] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupDependentUniforms >() {
                    Some( o ) => {
                        println!("draw group dependent uniforms");
                        renderer.set_draw_group_uniforms( o._group_id, &o._uniform_ids[..] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupDispatch >() {
                    Some( o ) => {
                        println!("draw group dispatch");
                        let renderer_immut : & Renderer = & * renderer;
                        Renderer::drawcall_draw_group( renderer_immut, &[ o._group_id ] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                return Err( &"unmatching render command" )
            }
        }
        renderer._objs.borrow_mut().clear();
        Ok( () )
    }
    pub fn reset_draw_group_data( & self, group_indices: &[ usize ] ) -> Result< (), & 'static str > {
        for &i in group_indices {
            if i >= self._draw_groups.borrow_mut().len() {
                return Err( "object index out of range" )
            }
            let mut dg = self._draw_groups.borrow_mut();
            dg[ i ].clear_buff_data();
        }
        Ok( () )
    }
    pub fn bind_draw_group_data( & self, group_indices: &[ usize ] ) -> Result< (), & 'static str > {
        for &i in group_indices {
            if i >= self._draw_groups.borrow_mut().len() {
                return Err( "object index out of range" )
            }
            match self._draw_groups.borrow_mut()[ i ].bind_buffer() {
                Err( e ) => return Err( e ),
                _ => (),
            }
        }
        Ok( () )
    }
    pub fn drawcall_draw_group( renderer: & Renderer, group_indices: &[ usize ] ) -> Result< (), & 'static str > {
        for &i in group_indices {
            for uniform_group in renderer._draw_group_uniforms.borrow()[i].iter() {
                println!("dispatching uniform group: {}", *uniform_group );
                if i >= renderer._draw_groups.borrow().len() {
                    return Err( "object index out of range" )
                }
                match renderer._uniforms.borrow_mut().send_uniform_group( *uniform_group ){
                    Err(e) => return Err(e),
                    _ => ()
                }
                match renderer._draw_groups.borrow_mut()[ i ].draw_buffer_all() {
                    Err( e ) => return Err( e ),
                    _ => (),
                }                
                // index += 1;
            }
        }
        Ok( () )
    }
    pub fn add_renderpass< T > ( & mut self, name: String, rp: T ) -> usize
    where T: i_renderpass::IRenderPass + 'static {
        let i = self._rp.len();
        self._map_string_to_rp.insert( name, i );
        self._rp.push( Box::new( rp ) );
        i
    }
    pub fn get_renderpass( & mut self, name: String ) -> Option< & mut i_renderpass::IRenderPass > {
        match self._map_string_to_rp.get( &name ){
            None => None,
            Some( o ) => {
                Some( self._rp[*o].deref_mut() )
            }
        }
    }
    // pub fn uniforms_ref( & mut self ) -> & mut renderdevice_gl::RenderUniformCollection {
    //     & mut self._uniforms.get_mut()
    // }
    pub fn add_draw_group_uniforms( & self, draw_group: usize, uniform_group: &[u64] ) -> Result< (), & 'static str > {
        if self._draw_group_uniforms.borrow_mut().len() <= draw_group {
            self._draw_group_uniforms.borrow_mut().resize( draw_group + 1, vec![] );
        }
        self._draw_group_uniforms.borrow_mut()[ draw_group ].extend_from_slice( uniform_group );
        Ok( () )
    }
    pub fn set_draw_group_uniforms( & self, draw_group: usize, uniform_group: &[u64] ) -> Result< (), & 'static str > {
        if self._draw_group_uniforms.borrow_mut().len() <= draw_group {
            self._draw_group_uniforms.borrow_mut().resize( draw_group + 1, vec![] );
        }
        self._draw_group_uniforms.borrow_mut()[ draw_group ].clear();
        self._draw_group_uniforms.borrow_mut()[ draw_group ].extend_from_slice( uniform_group );
        Ok( () )
    }
    pub fn clear_draw_group_uniforms( & self, draw_group: usize ) -> Result< (), & 'static str > {
        if self._draw_group_uniforms.borrow_mut().len() <= draw_group {
            return Err( "draw group out of range" )
        }
        self._draw_group_uniforms.borrow_mut()[ draw_group ].clear();
        Ok( () )
    }
    pub fn win_ref( & mut self ) -> & mut WinGlutin {
        & mut self._win
    }
}

