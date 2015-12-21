#include "PassType_ShadowMap_OpGL.h"

#include <cassert>

using namespace std;

bool PassType_ShadowMap_OpGL::Render( GLSLProgram *, Camera, Lights, Poly ){
    bool bRet = true;

    
    return bRet;
}

bool PassType_ShadowMap_OpGL::ProcessPassDepth( GLSLProgram *, Camera, Lights, Poly ){
    bool bRet = true;

    
    return bRet;
}

bool PassType_ShadowMap_OpGL::ProcessPassNormal( GLSLProgram *, Camera, Lights, Poly ){
    bool bRet = true;

    
    return bRet;
}

bool PassType_ShadowMap_OpGL::ProcessPassCommon( GLSLProgram * glsl_program, Camera camera, Lights lights, Poly poly ){
    bool bRet = true;

    if( !glsl_program ){
	assert( 0 && "GLSLProgram invalid in GLRenderPassShadowMap::ProcessPassDepth()");
	return false;
    }

    mat4 ModelViewMatrix;
    mat4 MVP;
    mat4 MVPB;
    mat3 NormalMatrix;
    if ( !render_mesh_orientation.GetCompositeMats( ModelViewMatrix, MVP, MVPB, NormalMatrix ) ){
	assert( 0 && "GetCompositeMats() failed in GLRenderPassShadowMap::ProcessPassDepth()");
	return false;
    }

    bRet = glsl_program->SetUniform( "ShadowMatrix", (mat4 const) MVPB );
    bRet = glsl_program->SetUniform( "MVP", (mat4 const) MVP );
    bRet = glsl_program->SetUniform( "ModelViewMatrix", (mat4 const) ModelViewMatrix );
    bRet = glsl_program->SetUniform( "NormalMatrix", (mat3 const) NormalMatrix );
    mat4 LightViewMatrix = render_mesh_orientation._MatView;
    bRet = glsl_program->SetUniform( "LightViewMatrix", (mat4 const) LightViewMatrix );
    glsl_program->BindVertexArray();
    for( auto & i : buffer_obj_name ){
	if( !glsl_program->SetCurrentBufferInfo( i ) ){
	    assert( 0 && "GLSLProgram::DrawCurrentBufferSegment() failed in GLRenderPassShadowMap::ProcessPassDepth()" );
	    return false;
	}
	if( !glsl_program->DrawCurrentBufferSegment() ){
	    assert( 0 && "GLSLProgram::DrawCurrentBufferSegment() failed in GLRenderPassShadowMap::ProcessPassDepth()" );
	    return false;
	}
    }
    glsl_program->UnBindVertexArray();

    return bRet;
}