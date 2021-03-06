#define CATCH_CONFIG_MAIN  // This tells Catch to provide a main() - only do this in one cpp file

#include <thread>
#include <vector>
#include <iostream>
#include <mutex>
#include <set>

#include "catch.hpp"
#include "hashtable_lock_striped.hpp"

using namespace std;
using namespace e2::dsc;

TEST_CASE( "hashtable_lock_striped", "[hashtable_lock_striped]" ) { 
    SECTION( "add" ) {
	hashtable_lock_striped< int, int > hashtable((size_t)200, 0.33f);
	for( int i = 0; i < 1000; ++i ){
	    bool bRet = hashtable.insert( i, i*10 );
	    CHECK( bRet );
	    int val_query;
	    bRet = hashtable.find( i, val_query );
	    CHECK( bRet );
	    CHECK( (i*10 == val_query) );
	}
    }
    SECTION( "erase" ) {
    	hashtable_lock_striped< int, int > hashtable(200, 0.33);
    	int val_query;
        bool bRet = hashtable.insert( 14, 100 );
    	bRet = hashtable.find( 14, val_query );
    	CHECK( bRet );
    	CHECK( 100 == val_query );
	
    	bRet = hashtable.erase( 14 );
    	CHECK( bRet );

    	bRet = hashtable.find( 14, val_query );
    	CHECK( false == bRet );
    }
    SECTION( "resize" ) {
    	hashtable_lock_striped< int, int > hashtable(10, 0.33);
    	for( int i = 0; i < 10; ++i ){
    	    bool bRet = hashtable.insert( i, i*10 );
    	    CHECK( bRet );
    	    int val_query;
    	    bRet = hashtable.find( i, val_query );
    	    CHECK( bRet );
    	    CHECK( (i*10 == val_query) );
    	}
    	CHECK( (true == hashtable.resize(2)) );
    	for( int i = 0; i < 10; ++i ){
    	    int val_query;
    	    bool bRet = hashtable.find( i, val_query );
    	    CHECK( bRet );
    	    CHECK( (i*10 == val_query) );
    	}
    }
    SECTION( "load factor" ) {
    	hashtable_lock_striped< int, int > hashtable(200, 0.33);
    	double error = 0.0001;
    	CHECK( hashtable.get_load_factor() >= 0 - error );
    	CHECK( hashtable.get_load_factor() <= 0 + error );
	
    	for( int i = 0; i < 1000; ++i ){
    	    hashtable.insert( i, i*10 );
    	}

    	CHECK( hashtable.get_load_factor() >= 5 - error );
    	CHECK( hashtable.get_load_factor() <= 5 + error );

    	CHECK( (true == hashtable.resize(100)) );

    	CHECK( hashtable.get_load_factor() >= 10 - error );
    	CHECK( hashtable.get_load_factor() <= 10 + error );
    }
}
