//---based on Art of Multiprocessor Programming section 9.8
#ifndef LIST_LOCKFREE_TOTAL_H
#define LIST_LOCKFREE_TOTAL_H

#include <cstring>
#include <atomic>
#include "i_list.hpp"

template< class T >
class list_lockfree_total_impl {
public:
    using _t_size = size_t;
    using _t_val = T;
    class Node;
    using _t_node = std::atomic< Node * >;
    using find_result = std::pair< Node *, Node * >; //stores previous and next nodes of found node

              class Node {
              public:
                     _t_node _next;
                      _t_val _val;
 		      size_t _key;
 		        bool _is_head;
 		        bool _is_marked;
		             Node(): _next( nullptr ), _is_head( false ), _is_marked( false ) {}
		             Node( size_t key, _t_val const & val ): _key( key ), _val( val ), _next( nullptr ), _is_head( false ), _is_marked( false ) {}
	      };

               list_lockfree_total_impl();
               ~list_lockfree_total_impl();
          bool clear();
          bool empty();
       _t_size size();
          bool add( _t_val const & val, size_t key );
          bool remove( _t_val & val, size_t key );
          bool contains( _t_val const & val, size_t key );
private:
   find_result find_window( _t_node & head, size_t key );
       _t_node _head;
};

#include "list_lockfree_total.tpp"

template< class T >
using list_lockfree_total = i_list < T, list_lockfree_total_impl,
				   trait_list_size::unbounded,
				   trait_list_concurrency::lockfree,
				   trait_list_method::total
				   >;

#endif