#include "GLIncludes.hpp"

#include "InitGL.hpp"
#include "IInit.hpp"

#include "WindowInfo.hpp"

#include <string>
#include <cstring>
#include <iostream>
#include <fstream>
#include <cstdio>
#include <initializer_list>

InitGL::~InitGL(){
    if( _window ){
	glfwDestroyWindow( _window );
	_window = nullptr;
    }
    glfwTerminate();
}

void InitGL::print_info_opengl(){
    const GLubyte * renderer = glGetString( GL_RENDERER );
    const GLubyte * vendor = glGetString( GL_VENDOR );
    const GLubyte * version = glGetString( GL_VERSION );
    const GLubyte * glslVersion = glGetString( GL_SHADING_LANGUAGE_VERSION );
    
    GLint major, minor;
    glGetIntegerv(GL_MAJOR_VERSION, &major);
    glGetIntegerv(GL_MINOR_VERSION, &minor);
    printf("GL Vendor: %s\n", vendor);
    printf("GL Renderer : %s\n", renderer);
    printf("GL Version (string) : %s\n", version);
    printf("GL Version (integer) : %d.%d\n", major, minor);
    printf("GLSL Version : %s\n", glslVersion);
}
bool InitGL::init( std::initializer_list<unsigned> const & window_args ){
    if ( !glfwInit() ) {
    	printf( "failed to initialize GLFW.\n" );
    	return -1;
    }

    if( window_args.size() < 2 ){
	std::cout << "window_args not specified. Using default 500x500." << std::endl;
	_width = 500;
	_height = 500;
    }else{
	size_t arg_count = 0;
	for( auto & i : window_args ){
	    if( arg_count == 0 )
		_width = i;
	    else
		_height = i;
	    ++arg_count;
	}

    }
    
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    _window = glfwCreateWindow( _width, _height, "Render Window", nullptr, nullptr );
    if ( !_window ) {
	return -1;
    }

    glfwMakeContextCurrent( _window );

    if (gl3wInit()) {
	printf( "failed to initialize OpenGL\n" );
	return -1;
    }

    print_info_opengl();

    glEnable( GL_DEPTH_TEST );
    glClearColor( 0, 0, 0, 1.0 );

    std::cout << "InitGL::init invoked." << std::endl;

    return true;
}

WindowInfo InitGL::GetWindowResource(){
    WindowInfo wininfo;
    wininfo._window = _window;
    wininfo._width = _width;
    wininfo._height = _height;
    wininfo._type = WindowType::GLFW;
    return wininfo;
}