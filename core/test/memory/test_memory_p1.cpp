#include <list>
#include <cstdint>
#include <cassert>
#include <vector>

#include "memory_p1.hpp"

#include "memory_partition_test_simple.hpp"

using namespace e2::memory;

int main(){
    
    memory_p1 mp;

    std::vector< void * > lookup_ptr;
    lookup_ptr.resize( 1000000, nullptr );
    
    memory_parition_test_simple::test_startup( &mp );

    memory_parition_test_simple::test_resize( &mp, 1000, &lookup_ptr );
    memory_parition_test_simple::test_resize( &mp, 500, &lookup_ptr );
    memory_parition_test_simple::test_resize( &mp, 10000, &lookup_ptr );
    memory_parition_test_simple::test_resize( &mp, 10, &lookup_ptr );
    memory_parition_test_simple::test_resize( &mp, 10, &lookup_ptr );
    memory_parition_test_simple::test_resize( &mp, 1000000, &lookup_ptr );

    memory_parition_test_simple::test_free( &mp );

    memory_parition_test_simple::test_resize( &mp, 25000, &lookup_ptr );

    memory_parition_test_simple::test_free( &mp );

    return 0;
}
