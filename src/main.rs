use c_simple_types::{on_off, print_distance, print_array, print_difference, ding};

//tengo las funciones en lib, pero el codigo principal (main) esta aca

pub fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    print_difference( coords.0 , coords.1);

    let coords_arr: [f32; 2] = [coords.0 , coords.1];

    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];

    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    //array[tuple.0]
    on_off(mess.2[1].0); // el tuple indexing se usa afuera del array, 2 es la posicion del elemento en mess(tupla), 1 es la posicion del , 0 es la posicion del array
}