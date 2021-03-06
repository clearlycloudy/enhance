#define CATCH_CONFIG_MAIN  // This tells Catch to provide a main() - only do this in one cpp file

#include <thread>
#include <vector>
#include <iostream>
#include <mutex>
#include <set>

#include "catch.hpp"
#include "list_search_lockfree_total.hpp"

using namespace std;
using namespace e2::dsc;

TEST_CASE( "list_search_lockfree_total single thread", "[list total]" ) { 
    list_search_lockfree_total<int> l;
    CHECK( l.size() == 0 );
    CHECK( l.empty() == true );
    size_t n = 20;
    for( size_t i = 0; i < n; ++i ){
	int val = i;
	bool bret = l.add( &val, i );
	CHECK( bret );
    }
    CHECK( l.size() == n );
    CHECK( l.empty() == false );
    for( size_t i = 0; i < n; ++i ){
	int val = i;
	bool bret = l.contains( &val, i );
	CHECK( bret );
    }
    for( size_t i = 0; i < n; ++i ){
	int val = -1;
	bool bret = l.remove( &val, i );
	CHECK( bret );
	CHECK( val == i );
    }
    CHECK( l.size() == 0 );
    CHECK( l.empty() == true );
}
