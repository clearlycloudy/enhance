#include "ParserMd5.hpp"
#include "IParser.hpp"
#include "file_md5_anim.hpp"
#include "file_md5_mesh.hpp"
#include "file_md5_skel.hpp"
#include "file_md5_calc_mesh.hpp"

#include <vector>
#include <list>
#include <string>
#include <cassert>
#include <iostream>
#include <utility>

using namespace std;

std::pair<bool, ParserMd5::md5_data > ParserMd5::parse( std::string path_mesh, std::vector<std::string> paths_anim ){
    std::pair<bool, file_md5_mesh::data_mesh> retmesh = file_md5_mesh::process( path_mesh );
    assert( retmesh.first );
    if( false == retmesh.first )
	return { false, {} };
	
    file_md5_mesh::data_mesh dmesh = std::move( retmesh.second );
    
    std::list< std::pair< md5_anim_info, file_md5_skel::skel_collection > > scs {};

    for( auto p : paths_anim ){
	
	std::pair<bool, file_md5_anim::data_anim> retanim = file_md5_anim::process( p );
	assert( retanim.first );
	if( false == retanim.first )
	    return { false, {} };
    
	file_md5_anim::data_anim danim = std::move( retanim.second );

	std::cout << "data_anim: md5version: " << danim._md5version << std::endl;
	std::cout << "data_anim: commandline: " << danim._commandline << std::endl;
	std::cout << "data_anim: numframes: " << danim._numframes << std::endl;
	std::cout << "data_anim: numjoints: " << danim._numjoints << std::endl;
	std::cout << "data_anim: framerate: " << danim._framerate << std::endl;
	std::cout << "data_anim: num_animated_components: " << danim._num_animated_components << std::endl;

	std::pair<bool, file_md5_skel::skel_collection> retskel = file_md5_skel::process( danim );
	assert( retskel.first );
	if( false == retskel.first )
	    return { false, {} };

	file_md5_skel::skel_collection sc = std::move( retskel.second );
	if( sc._skels.size() != danim._numframes )
	{
	    assert( sc._skels.size() == danim._numframes );
	    return { false, {} };
	}
	md5_anim_info anim_info;
	anim_info._numframes = danim._numframes;
	anim_info._framerate = danim._framerate;
	scs.push_back( { anim_info, std::move(sc) } );
    }
    md5_data ret;
    ret._dm = std::move(dmesh);
    ret._scs = std::move(scs);
    return { true, ret };
}