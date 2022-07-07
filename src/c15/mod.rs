//! Smart Pointers

mod _15_1_using_box_t_to_point_to_data_on_the_heap;
mod _15_2_treating_smart_pointers_like_regular_references_with_the_deref_trait;
mod _15_3_running_code_on_cleanup_with_the_drop_trait;
mod _15_4_rc_t_the_reference_counted_smart_pointer;
mod _15_5_refcell_t_and_the_interior_mutability_pattern;
mod _15_6_reference_cycles_can_leak_memory;


pub use  _15_1_using_box_t_to_point_to_data_on_the_heap::main                               as main_15_1;
pub use  _15_2_treating_smart_pointers_like_regular_references_with_the_deref_trait::main   as main_15_2;
pub use  _15_3_running_code_on_cleanup_with_the_drop_trait::main                            as main_15_3;
pub use  _15_4_rc_t_the_reference_counted_smart_pointer::main                               as main_15_4;
pub use  _15_5_refcell_t_and_the_interior_mutability_pattern::main                          as main_15_5;
pub use  _15_6_reference_cycles_can_leak_memory::main                                       as main_15_6;