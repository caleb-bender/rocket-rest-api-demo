
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::serde::json::Json;

    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]  
    #[serde(rename_all = "lowercase")]
    pub enum SortOrderType {
        Ascending,
        Descending
    }


    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]   
    pub struct SortPostData {
        array: Vec<i64>,
        sort_order: SortOrderType
    }

    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct SortPostResponse {
        sorted_array: Vec<i64>
    }

    pub fn sort_post_data_and_return_response(sort_post_data: Json<SortPostData>) -> SortPostResponse {
        let mut sorted_array = sort_post_data.array.clone();
        sorted_array.sort();
        if let SortOrderType::Descending = sort_post_data.sort_order {
            sorted_array.reverse();
        }
        
        SortPostResponse {
            sorted_array: sorted_array
        }
    }

    #[post("/", data = "<sort_post_data>")]
    pub fn sort(sort_post_data: Json<SortPostData>) -> Json<SortPostResponse> {
        Json(sort_post_data_and_return_response(sort_post_data))
    }



    #[cfg(test)]
    mod sorter_tests {

        use std::vec;
        use crate::sorter::*;
        use super::{sort_post_data_and_return_response, SortPostData};

        #[test]
        fn should_sort_ascending_array_correctly() {
            
            let sort_post_data = SortPostData {
                array: vec![6,4,1,2,5,3],
                sort_order: SortOrderType::Ascending
            };
            let sort_post_response = sort_post_data_and_return_response(Json(sort_post_data));

            assert_eq!(sort_post_response.sorted_array, vec![1,2,3,4,5,6]);
        }

        #[test]
        fn should_sort_descending_array_correctly() {
            
            let sort_post_data = SortPostData {
                array: vec![6,4,1,2,5,3],
                sort_order: SortOrderType::Descending
            };
            let sort_post_response = sort_post_data_and_return_response(Json(sort_post_data));

            assert_eq!(sort_post_response.sorted_array, vec![6,5,4,3,2,1]);
        }
    }