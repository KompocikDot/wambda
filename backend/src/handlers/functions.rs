// async fn create_function(
//     State(state): State<Arc<AppState>>,
//     mut multipart: Multipart,
// ) -> StatusCode {
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         if field.name() == Some("wasm") {
//             let data = field.bytes().await.unwrap();
//             let blob_uuid = Uuid::now_v7().to_string();

//             let insert = state
//                 .blob_bucket
//                 .put_object(&blob_uuid, &data.to_vec())
//                 .await;

//             return match insert {
//                 Ok(_) => StatusCode::CREATED,
//                 Err(err) => {
//                     eprintln!("{err}");
//                     StatusCode::INTERNAL_SERVER_ERROR
//                 }
//             };
//         }
//     }

//     return StatusCode::BAD_REQUEST;
// }

// async fn run(State(state): State<Arc<AppState>>, Json(payload): Json<RunPayload>) -> &'static str {
//     // let start_time = ActiveValue::Set(Utc::now().into());
//     let binary = state
//         .blob_bucket
//         .get_object(payload.binary_id)
//         .await
//         .unwrap();

//     let mut args = Vec::new();
//     for arg in payload.fn_args {
//         args.push(from_json(arg));
//     }

//     let out = execute_wasm(binary.as_slice(), args.as_slice()).unwrap();
//     println!("{:?}", out);
//     // let finish_time = ActiveValue::Set(Utc::now().into());

//     // let new_run = runs::ActiveModel {
//     //     start_time,
//     //     finish_time,
//     //     ..Default::default()
//     // };

//     // new_run.insert(&state.db).await.unwrap();
//     "hi, world!"
// }
