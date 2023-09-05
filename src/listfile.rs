use mpq::Archive;

use crate::utils::{ read_mpq_file_to_u8_vec, print_type_of };

pub fn read_list_file_data(archive: &mut mpq::Archive)->  {
  read_mpq_file_to_u8_vec("(listfile)").map()
}