mod main_data {
        pub const LENGTH: usize = 64;
}

struct CustomFloat {
        data: Vec<u8>,
        negative: bool
}


impl CustomFloat {
        fn new() -> CustomFloat {
                return CustomFloat {
                        data: generate_empty_vec(),
                        negative: false,
                };
        }
        fn new_with_number(input: f64) -> CustomFloat {
                return CustomFloat {
                        data: normalise(input),
                        negative: false,
                };
        }
        fn add(&mut self, input: f64) {
                let input_normalised: Vec<u8> = normalise(input);

                let (init_data, input_data) = move_comma(self.data.clone(), input_normalised);
                
                if self.negative {
                        self.negative = will_be_negative_add(&init_data, &input_data);
                }

                self.data = handle_add(init_data, input_data);
        }
        fn substract(&mut self, input: f64) {
                let input_normalised: Vec<u8> = normalise(input);
                let (init_data, input_data) = move_comma(self.data.clone(), input_normalised);
                
                self.negative = will_be_negative_substract(&init_data, &input_data);
                
                if self.negative {
                        self.data = handle_substract_negative(init_data, input_data);
                } else {
                        self.data = handle_substract(init_data, input_data);
                }
        }
        fn print(&self) {
                let mut data_to_print: String = String::new();
                for index in 0..main_data::LENGTH {
                        if self.data[index] == 10 {
                                data_to_print.push(',');
                                continue;
                        }
                        data_to_print.push_str(&self.data[index].to_string());
                }
                remove_non_necessary(&mut data_to_print);
                if self.negative {
                        data_to_print = format!("-{}", data_to_print);
                }
                println!("{}", data_to_print);
        }
}

fn handle_add(init_data: Vec<u8>, input_data: Vec<u8>) -> Vec<u8> {
        let get_buffer = |n1: u8, n2: u8| {
                if n1 + n2 >= 10 {
                        return 1;
                } else {
                        return 0;
                }
        };
        let get_number_to_add = |n1: u8, n2: u8, buffer: u8| {
                let added: u8 = n1 + n2 + buffer;
                if added >= 10 {
                        return added - 10;
                } else {
                        return added;
                }
        };

        let mut buffer: u8 = 0 as u8;
        let mut vec_to_return: Vec<u8> = generate_vec();
        for index in 0..main_data::LENGTH {
                let real_index = ((index as i32 - (main_data::LENGTH - 1) as i32) as i32).abs();
                let number_from_data: u8 = init_data[real_index as usize];
                let number_from_input: u8 = input_data[real_index as usize];
                if number_from_data == 0 && number_from_input == 0 && buffer == 0 {
                        continue;
                }
                if number_from_data == 10 {
                        vec_to_return[real_index as usize] = 10;
                        continue; // komma
                }

                vec_to_return[real_index as usize] = get_number_to_add(number_from_data, number_from_input, buffer);

                buffer = get_buffer(number_from_data, number_from_input);
        }
        return vec_to_return;
}

fn handle_substract(mut init_data: Vec<u8>, input_data: Vec<u8>) -> Vec<u8> {
        let can_substract = |n1: i32, n2: i32| {
                if n1 - n2 < 0 {
                        return false;
                } else {
                        return true;
                }
        };

        let mut data_to_return: Vec<u8> = generate_empty_vec();
        for index in 0..main_data::LENGTH {
                let real_index = ((index as i32 - (main_data::LENGTH - 1) as i32) as i32).abs();
                let number_from_data: u8 = init_data[real_index as usize];
                let number_from_input: u8 = input_data[real_index as usize];
                if number_from_data == 0 && number_from_input == 0 {
                        data_to_return[real_index as usize] = 0;
                        continue;
                }
                if number_from_data == 10 {
                        data_to_return[real_index as usize] = 10;
                        continue; // komma
                }

                if can_substract(number_from_data as i32, number_from_input as i32) {
                        data_to_return[real_index as usize] = number_from_data - number_from_input;
                } else {
                        if init_data[real_index as usize - 1] == 10 {
                                init_data[real_index as usize - 2] = init_data[real_index as usize - 2] - 1;
                        } else {
                                init_data[real_index as usize - 1] = init_data[real_index as usize - 1] - 1;
                        }
                        data_to_return[real_index as usize] = (number_from_data + 10) - number_from_input;
                }
        }
        return data_to_return;
}
fn handle_substract_negative(init_data: Vec<u8>, mut input_data: Vec<u8>) -> Vec<u8> {
        let can_substract = |n1: i32, n2: i32| {
                if n1 - n2 < 0 {
                        return false;
                } else {
                        return true;
                }
        };

        let mut data_to_return: Vec<u8> = generate_empty_vec();
        for index in 0..main_data::LENGTH {
                let real_index = ((index as i32 - (main_data::LENGTH - 1) as i32) as i32).abs();
                let number_from_data: u8 = init_data[real_index as usize];
                let number_from_input: u8 = input_data[real_index as usize];
                if number_from_data == 0 && number_from_input == 0 {
                        data_to_return[real_index as usize] = 0;
                        continue;
                }
                if number_from_data == 10 {
                        data_to_return[real_index as usize] = 10;
                        continue; // komma
                }

                if can_substract(number_from_input as i32, number_from_data as i32) {
                        data_to_return[real_index as usize] = number_from_input - number_from_data;
                } else {
                        if input_data[real_index as usize - 1] == 10 {
                                input_data[real_index as usize - 2] = input_data[real_index as usize - 2] - 1;
                        } else {
                                let mut iterator: usize = real_index as usize - 1;
                                loop {
                                        if input_data[iterator] == 0 {
                                                input_data[iterator] = 9;
                                                iterator -= 1;
                                        } else if input_data[iterator] == 10 {
                                                iterator -= 1;
                                                continue;
                                        } else {
                                                input_data[iterator] -= 1;
                                                break;
                                        }
                                }
                                input_data[real_index as usize - 1] = input_data[real_index as usize - 1] - 1;
                        }
                        data_to_return[real_index as usize] = (number_from_input + 10) - number_from_data;
                }
        }
        return data_to_return;
}

fn will_be_negative_substract(data: &Vec<u8>, input: &Vec<u8>) -> bool {
        let mut is_negative: bool = false;
        for index in 0..main_data::LENGTH {
                if input[index as usize] > data[index as usize] {
                        is_negative = true;
                        break;
                } else if input[index as usize] == data[index as usize] {
                        continue;
                } else {
                        break;
                }
        }
        return is_negative;
}
fn will_be_negative_add(data: &Vec<u8>, input: &Vec<u8>) -> bool {
        let mut is_negative: bool = true;
        for index in 0..main_data::LENGTH {
                if input[index as usize] > data[index as usize] {
                        is_negative = false;
                        break;
                } else if input[index as usize] == data[index as usize] {
                        continue;
                } else {
                        break;
                }
        }
        return is_negative;
}

fn remove_non_necessary(input: &mut String) {
        remove_ending_zeros(input);
        if input.len() == 1 {
                return;
        }
        remove_starting_zeros(input);
        remove_last_comma(input);
}

fn remove_ending_zeros(input: &mut String) {
        loop {
                if input.chars().last().unwrap() == '0' {
                        input.pop();
                } else {
                        break;
                }
                if input.len() == 1 {
                        break;
                }
        }
}
fn remove_starting_zeros(input: &mut String) {
        loop {
                if input.chars().next().unwrap() == '0' {
                        input.remove(0);
                } else {
                        break;
                }
        }
}
fn remove_last_comma(input: &mut String) {
        if input.chars().last().unwrap() == ',' {
                input.pop();
        }
}

fn generate_vec() -> Vec<u8> {
        let mut vec_to_return: Vec<u8> = Vec::<u8>::with_capacity(main_data::LENGTH);
        for _ in 0..main_data::LENGTH {
                vec_to_return.push(0);
        }
        return vec_to_return;
}
fn generate_empty_vec() -> Vec<u8> {
        let mut vec_to_return: Vec<u8> = generate_vec();
        vec_to_return[1] = 10;
        return vec_to_return;
}

fn move_comma(init: Vec<u8>, input: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        let mut datas: [Vec<u8>; 2] = [init.clone(), input.clone()];

        let (length_init, length_input) = count_diff_comma(datas[0].clone(), datas[1].clone());
        let length_to_extend: u8 = (length_init as i32 - length_input as i32).abs() as u8;
        let final_length: u8 = main_data::LENGTH as u8 - 1 - length_to_extend;

        if length_init > length_input {
                for index in 0..(final_length + 1) {
                        let real_index: usize = (index as i32 - final_length as i32).abs() as usize;
                        datas[1][real_index + length_to_extend as usize] = datas[1][real_index];
                }
                for index in 0..length_to_extend {
                        datas[1][index as usize] = 0;
                }
        } else if length_init < length_input {
                for index in 0..(final_length + 1) {
                        let real_index: usize = (index as i32 - final_length as i32).abs() as usize;
                        datas[0][real_index + length_to_extend as usize] = datas[0][real_index];
                }
                for index in 0..length_to_extend {
                        datas[0][index as usize] = 0;
                }
        }
        // else :
        //   they are the same length, nothings to do c:
        

        return (datas[0].clone(), datas[1].clone());
}

fn count_diff_comma(n1: Vec<u8>, n2: Vec<u8>) -> (u8, u8) {
        let mut buffer: [u8; 2] = [0 as u8; 2];
        let mut iterator: usize = 0;
        let mut stop_count_n1: bool = false;
        let mut stop_count_n2: bool = false;
        loop {
                if !stop_count_n1 {
                        if n1[iterator] == 10 {
                                stop_count_n1 = true;
                        } else {
                                buffer[0] += 1;
                        }
                }
                if !stop_count_n2 {
                        if n2[iterator] == 10 {
                                stop_count_n2 = true;
                        } else {
                                buffer[1] += 1;
                        }
                }
                if stop_count_n1 && stop_count_n2 {
                        break;
                }
                
                iterator += 1;
        }

        return (buffer[0] as u8, buffer[1] as u8);
}

fn normalise(input: f64) -> Vec<u8> {
        let mut data_to_return: Vec<u8> = generate_vec();
        let mut iterator: usize = 0;
        let mut hash_komma_been_placed: bool = false;
        for cell in input.to_string().chars() {
                if cell == '.' {
                        hash_komma_been_placed = true;
                        data_to_return[iterator] = 10 as u8;
                } else {
                        data_to_return[iterator] = cell.to_string().parse::<u8>().unwrap_or(0);
                }
                iterator += 1;
        }
        if !hash_komma_been_placed {
                data_to_return[iterator] = 10;
        }
        return data_to_return;
}



fn main() {
        let mut custom_float: CustomFloat = CustomFloat::new();
        custom_float.print();
        

        custom_float.substract(10.0);
        custom_float.print();


        custom_float.add(110.0);
        custom_float.print();
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn assert_remove_ending_zeros() {
                let mut test: String = String::from("00100");
                remove_ending_zeros(&mut test);

                assert_eq!(test, "001");
        }
        #[test]
        fn assert_remove_starting_zeros() {
                let mut test: String = String::from("00100");
                remove_starting_zeros(&mut test);

                assert_eq!(test, "100");
        }
        #[test]
        fn assert_remove_last_comma() {
                let mut test: String = String::from("100,");
                remove_last_comma(&mut test);

                assert_eq!(test, "100");
        }
        #[test]
        fn assert_count_diff_comma() {
                let vec_1: Vec<u8> = [1,0,0,10,9,2].to_vec();
                let vec_2: Vec<u8> = [5,6,10,8].to_vec();
                let (out_1, out_2) = count_diff_comma(vec_1, vec_2);

                assert_eq!(out_1, 3);
                assert_eq!(out_2, 2);
        }
        #[test]
        fn assert_move_comma() {
                let mut vec_1: Vec<u8> = [1,6,8,2,10,5,7,1].to_vec();
                let mut vec_2: Vec<u8> = [5,1,7,10,8,9].to_vec();
                let mut out_1: Vec<u8> = [1,6,8,2,10,5,7,1].to_vec();
                let mut out_2: Vec<u8> = [0,5,1,7,10,8,9].to_vec();
                for _ in vec_1.len()..main_data::LENGTH {
                        vec_1.push(0);
                        out_1.push(0);
                }
                for _ in vec_2.len()..main_data::LENGTH {
                        vec_2.push(0);
                }
                for _ in out_2.len()..main_data::LENGTH {
                        out_2.push(0);
                }

                assert_eq!(move_comma(vec_1, vec_2), (out_1, out_2));
        }
        #[test]
        fn assert_normalise() {
                let input: f64 = 3782.1934;
                let mut out: Vec<u8> = [3,7,8,2,10,1,9,3,4].to_vec();
                for _ in out.len()..main_data::LENGTH {
                        out.push(0);
                }

                assert_eq!(normalise(input), out);
        }
}