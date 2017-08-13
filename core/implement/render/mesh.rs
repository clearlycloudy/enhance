use implement::math::mat::Mat4;
use implement::math::mat::Mat3x1;
use implement::math::mat::Mat2x1;
use implement::render::camera;
use std::collections::HashMap;
use std::vec::Vec;

extern crate gl;

use interface::i_ele;
use interface::i_renderobj;
use implement::render::renderdevice_gl;
use implement::render::util_gl;

#[derive(Clone)]
pub struct Mesh {
    pub _id: u64,
    pub _pos: Vec< Mat3x1< f32 > >,
    pub _normal: Vec< Mat3x1< f32 > >,
    pub _tc: Vec< Mat2x1< f32 > >,
    // _xform: Mat4< f32 >,
}

impl Mesh {
    pub fn init( id: u64 ) -> Mesh {
        Mesh {
            _id: id,
            _pos: vec![],
            _normal: vec![],
            _tc: vec![],
        }
    }
}

impl i_ele::Facility for Mesh {
    // type Concrete = Mesh;
}

impl i_renderobj::IRenderable for Mesh {
    fn get_render_method( & self ) -> i_renderobj::RenderMethod {
        i_renderobj::RenderMethod::ADS
    }
}

impl i_renderobj::IRenderBuffer for Mesh {
    fn load_into_buffer( & mut self, rd: & mut i_renderobj::RenderDevice ) -> Result< (), & 'static str > {
        if self._pos.len() != self._normal.len() ||
           self._pos.len() != self._tc.len()
        {
            return Err( &"unequal length for position, normal, tc data" )
        }

        let mut pos = vec![];
        let mut normal = vec![];
        let mut tc = vec![];
        for i in 0..self._pos.len() {
            pos.extend_from_slice( &self._pos[i]._val[..] );
        }
        for i in 0..self._normal.len() {
            normal.extend_from_slice( &self._normal[i]._val[..] );
        }
        for i in 0..self._tc.len() {
            tc.extend_from_slice( &self._tc[i]._val[..] );
        }

        let data_map : HashMap< i_renderobj::BuffDataType, &[f32] > =  [ ( i_renderobj::BuffDataType::POS, pos.as_slice() ),
                                                                         ( i_renderobj::BuffDataType::NORMAL, normal.as_slice() ),
                                                                         ( i_renderobj::BuffDataType::TC, tc.as_slice() ) ].iter().cloned().collect();
        rd.store_buff_data( i_renderobj::RenderObjType::TRI, & data_map );

        println!( "load into render buffer: mesh: vertex count:{}", pos.len() / 3 );
        Ok( () )
    }
}

impl i_renderobj::IRenderUniform for Mesh {
    fn load_into_uniform( & mut self, uniforms: & mut renderdevice_gl::RenderUniformCollection ) -> Result< (), & 'static str > {
        Ok( () )
    }
}
