extern crate glutin;
extern crate image;
extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use interface::i_ele;
use interface::i_window::IWindow;
use interface::i_game_logic::IGameLogic;
use interface::i_renderer::IRenderer;

use implement::window::winglutin::WinGlutin;
use implement::kernel::kernel;
use implement::kernel::kernel_render;
use implement::render::util_gl;
use implement::render::texture;
use implement::render::camera;
use implement::render::light;
use implement::render::mesh;
use implement::render::primitive;
use implement::math;

use self::rand::Rng;
use self::image::GenericImage;

pub fn file_open( file_path: & str ) -> Option<String> {
    let path = File::open( file_path ).expect("file path open invalid");
    let mut buf_reader = BufReader::new(path);
    let mut contents = String::new();
    match buf_reader.read_to_string( & mut contents ){
        Err( e ) => { error!("{}", e ); return None },
        _ => (),
    }
    Some(contents)
}

pub struct GameLogic {
    //todo
    _is_init: bool,
    _lights: Vec< light::LightAdsPoint >, //could move this to be generated by game logic or within a connecting adaptor between game logic and render interface
    _cameras: Vec< camera::Cam >, //could move this to be generated by game logic or within a permanant cache for long term data storage
    _delta: f32, //test parameter for object velocity
    _path_shader_vs: String,
    _path_shader_fs: String,
}

impl IGameLogic for GameLogic {
    type EventInput = glutin::Event;
    type EventRender = kernel_render::Event;
    fn init() -> GameLogic {
        let mut ret = GameLogic {
            _is_init: false,
            _lights: vec![],
            _cameras: vec![],
            _delta: 0f32,
            _path_shader_vs: String::from("core/asset/shader/ads.vs"), //some hard coded paths for now
            _path_shader_fs: String::from("core/asset/shader/ads.fs"),
        };
        
        //lights
        let mut rng = rand::thread_rng();
        for i in 0..50 {
            let pos_x = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 - 3f32;
            let pos_y = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 - 4f32;
            let pos_z = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 + 10f32;
            let colour_r = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let colour_g = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let colour_b = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let l = light::LightAdsPoint {
                _id: i as u64,
                _pos: math::mat::Mat3x1 { _val: [ pos_x, pos_y, pos_z ] },
                _ads_val_spec: math::mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
                _ads_val_diff: math::mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
                _ads_val_amb: math::mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
            };
            ret._lights.push( l );
        }

        //camera
        let fov = 120f32;
        let aspect = 1f32;
        let near = 0.001f32;
        let far = 1000f32;
        let cam_foc_pos = math::mat::Mat3x1 { _val: [0f32, 0f32, 5f32] };
        let cam_up = math::mat::Mat3x1 { _val: [0f32, 1f32, 0f32] };
        let cam_pos = math::mat::Mat3x1 { _val: [5f32, 5f32, 20f32] };
        let cam_id = 0;
        let cam = camera::Cam::init( cam_id, fov, aspect, near, far, cam_pos, cam_foc_pos, cam_up );
        ret._cameras.push( cam );

        ret
    }
    fn process_input_events( & mut self, e: & [ Self::EventInput ] ) -> ( Vec< Self::EventRender >, bool ) {

        let mut v = vec![];
        if !self._is_init {
            //first time initilization
            
            info!("game logic: first time initialization.");
            
            self._is_init = true;
            let vs_src = file_open( self._path_shader_vs.as_str() ).expect("vertex shader not retrieved");
            let fs_src = file_open( self._path_shader_fs.as_str() ).expect("fragment shader not retrieved");
            let event_load_shader = kernel_render::Event::LoadShader(
                vec![
                    ( vs_src, util_gl::ShaderType::VERTEX ),
                    ( fs_src, util_gl::ShaderType::FRAGMENT ),
                ] );
            v.push( event_load_shader );

            let img = image::open( &Path::new( "core/asset/images/texture0.jpg" ) ).unwrap();
            debug!( "image dimension: {:?}", img.dimensions() );
            debug!( "image type: {:?}", img.color() );
            
            let texture0 = texture::Texture::from( &img );
            let texture_data = Vec::from( texture0 );
            let ( w, h ) = img.dimensions();
            let event_load_texture = kernel_render::Event::LoadTexture( String::from("texture0"), texture_data, w as _, h as _ );
            v.push( event_load_texture );

        }

        //input events
        let mut sig_exit = false;
        let mut new_win_dim = None;
        trace!("events input: {:?}", e );
        for input_event in e.iter() {
            match input_event {
                &glutin::Event::WindowEvent{ ref event, .. } => match event {
                    &glutin::WindowEvent::Closed => {
                        sig_exit = true;
                        break;
                    },
                    &glutin::WindowEvent::Resized(w, h) => new_win_dim = Some( (w,h) ),
                    &glutin::WindowEvent::ReceivedCharacter(x) => {
                        match x {
                            'q' => {
                                info!("events input received character: {:?}", x );
                                sig_exit = true;
                                break;
                            },
                            _ => (),
                        }
                    },
                    // &glutin::WindowEvent::KeyboardInput {
                    //     input: glutin::KeyboardInput {
                    //         state: glutin::ElementState::Pressed,
                    //         virtual_keycode: Some( glutin::VirtualKeyCode::Q ),
                    //         ..
                    //     }, ..
                    // } => {
                    //     info!("events input: {:?}", input_event );
                    //     sig_exit = true; //signal to exit
                    //     break;
                    // },
                    _ => (),
                },
                _ => ()
            }
        }
        //todo: make an event for this 
        if let Some( ( _w, _h ) ) = new_win_dim {
            unimplemented!();
            //win._win._wingl.resize(w, h);
        }
        
        //create some meshes
        //set triangle vert positions and normals
        let mut mesh = mesh::Mesh::init( 0 );
        mesh._pos.extend_from_slice( &[ math::mat::Mat3x1 { _val: [-1f32, -1f32, -1f32 ] },
                                        math::mat::Mat3x1 { _val: [ 5f32, -1f32, -1f32 ] },
                                        math::mat::Mat3x1 { _val: [-1f32,  1f32, -1f32 ] },
                                        math::mat::Mat3x1 { _val: [ 4f32, -1f32, 15f32 ] },
                                        math::mat::Mat3x1 { _val: [ 6f32, -1f32, 15f32 ] },
                                        math::mat::Mat3x1 { _val: [ 4f32,  1f32, 15f32 ] }, ] );

        mesh._normal.extend_from_slice( &[ math::mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                           math::mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                           math::mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                           math::mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                           math::mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                           math::mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] }, ] );
        
        mesh._tc.extend_from_slice( &[ math::mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                       math::mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                       math::mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                       math::mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                       math::mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                       math::mat::Mat2x1 { _val: [ 0f32, 0f32 ] }, ] );

        let mesh_copy = mesh.clone();

        let mut mesh2 = mesh_copy.clone();
        mesh2._pos.clear();
        mesh2._pos.extend_from_slice( &[ math::mat::Mat3x1 { _val: [-1f32+self._delta, -1f32, -1f32 ] },
                                         math::mat::Mat3x1 { _val: [ 5f32+self._delta, -1f32, -1f32 ] },
                                         math::mat::Mat3x1 { _val: [-1f32+self._delta,  1f32, -1f32 ] },
                                         math::mat::Mat3x1 { _val: [ 4f32+self._delta, -1f32, 15f32 ] },
                                         math::mat::Mat3x1 { _val: [ 6f32+self._delta, -1f32, 15f32 ] },
                                         math::mat::Mat3x1 { _val: [ 4f32+self._delta,  1f32, 15f32 ] }, ] );
        v.push( kernel_render::Event::AddObj( i_ele::Ele::init( mesh2 ) ) );

        let prim_box = primitive::Poly6 { _pos: math::mat::Mat3x1 { _val: [ -5f32, -10f32, 5f32 ] },
                                           _radius: 5f32 };

        v.push( kernel_render::Event::AddObj( i_ele::Ele::init( prim_box ) ) );

        let prim_sphere = primitive::SphereIcosahedron::init( math::mat::Mat3x1 { _val: [ -20f32, -10f32, 0f32 ] }, 5f32 );

        v.push( kernel_render::Event::AddObj( i_ele::Ele::init( prim_sphere ) ) );
        
        let l = &self._lights[0];
        v.push( kernel_render::Event::AddObj( i_ele::Ele::init( l.clone() ) ) );

        v.push( kernel_render::Event::AddObj( i_ele::Ele::init( self._cameras[0].clone() ) ) );

        self._delta += -0.01;

        ( v, sig_exit )
    }
}

pub type KernelImpl000 = kernel::Kernel < GameLogic, kernel_render::Renderer, WinGlutin, glutin::Event, kernel_render::Event, KernelImpl000Hooks >;


pub struct KernelImpl000Hooks {}

impl < G, R, W, EInput, ERender > kernel::KernelImplHooks < G, R, W, EInput, ERender >
    for KernelImpl000Hooks 
    where G: IGameLogic< EventInput = EInput, EventRender = ERender >,
          R: IRenderer< EventRender = ERender >,
          W: IWindow< EventType = EInput >
{
    fn init() -> Self {
        KernelImpl000Hooks {}
    }
    fn init_hooks( & mut self, windowing: & mut W, _game_logic: & mut G, renderers: & mut Vec< R > ) -> Result< (), & 'static str > {
        windowing.make_current()?;

        renderers.push( R::init().unwrap() );

        Ok( () )
    }
}    

