mod utils;
pub mod schemas;

use std::{str::Chars};
use schemas::LacOut;
use paddle_inference_rust_api::{PdConfig, PdPredictor};
use crate::utils::{load_word2id_dict, load_q2b_dict, load_id2label_dict};


fn parse_targets(labels: Vec<String>, words: Vec<String>) -> Vec<LacOut> {
    let mut result: Vec<LacOut> = vec![];

    labels.iter().enumerate().for_each(|(i, label)| {
        let label_len = label.len();

        if result.is_empty() || label.rfind("B") == Some(label_len  - 1) || label.rfind("S") == Some(label_len - 1) {
            let tag = &labels[i][0..(labels[i].len() - 2)];

            result.push(LacOut {
                word: words[i].clone(),
                tag: tag.to_owned(),
            });
        }
        else {
            let result_len = result.len();
            result[result_len - 1].word.push_str(&words[i]);
        }
    });

    result
}

pub fn run(query: &str) -> Vec<LacOut> {
    let model_path_raw = String::from("lac_model");
    let word2dict = load_word2id_dict(format!("{}/conf/word.dic", model_path_raw.clone()));
    let q2b_dict = load_q2b_dict(format!("{}/conf/q2b.dic", model_path_raw.clone()));
    let id2label_dict = load_id2label_dict(format!("{}/conf/tag.dic", model_path_raw.clone()));
    
    let _oov_id: i64 = word2dict.get("OOV")
        .unwrap_or(&(word2dict.len() as i64 - 1))
        .to_owned();

    println!("[baidu-lac-rs]: init lac model");

    let config: PdConfig = PdConfig::new();
    config.enable_mkldnn();
    config.disable_gpu();
    config.disable_glog_info();
    config.set_cpu_math_library_num_threads(1);
    config.set_model_dir("lac_model/model");

    println!("[baidu-lac-rs]: init config");

    let predictor = PdPredictor::new(&config);
    let input_names = predictor.get_input_names();
    let input_tensor = predictor.get_input_handle(&input_names[0]);
    let output_names = predictor.get_output_names();
    let output_tensor = predictor.get_output_handle(&output_names[0]);

    println!("[baidu-lac-rs]: init predictor");

    // generate data
    let mut _sec_words_batch: Vec<Chars> = vec![];
    let mut shape_size: u64 = 0;
    let sec_words = query.chars();
    _sec_words_batch.push(sec_words.clone());
    shape_size += sec_words.count() as u64;

    let mut c_lod_vec_: Vec<u64> = vec![0];
    c_lod_vec_.push(shape_size);
    
    let t_lod = vec![c_lod_vec_.clone()];
    input_tensor.set_lod(t_lod);

    let shape: Vec<i32> = vec![_sec_words_batch[0].clone().count() as i32, 1];
    input_tensor.reshape(shape);

    let data: Vec<i64> = _sec_words_batch[0].clone().into_iter().map(|c| {
        let mut word = c.clone().to_string();
        if let Some(q2b_word) = q2b_dict.get(&word) {
            word = q2b_word.to_owned();
        }

        let mut word_id: i64 = _oov_id;
        if let Some(dict_id) = word2dict.get(&word) {
            word_id = dict_id.to_owned();
        }

        word_id
    }).collect();

    println!("[baidu-lac-rs]: will copy_from_cpu");
    input_tensor.copy_from_cpu(data);
    println!("[baidu-lac-rs]: copy_from_cpu done");

    use std::time::Instant;
    let now = Instant::now();
    predictor.run();
    let elapsed = now.elapsed();
    println!("[baidu-lac-rs]: Elapsed: {:.2?}", elapsed);

    let output_shape = output_tensor.get_shape();
    let mut output_data: Vec<i64> = vec![0; output_shape[0].try_into().unwrap()];
    output_tensor.copy_to_cpu(&mut output_data);

    let labels: Vec<String> = output_data.iter().map(|label_id| {
        if let Some(label) = id2label_dict.get(&label_id) {
            return label.to_owned();
        }

        String::from("")
    }).collect();

    let out = parse_targets(labels, _sec_words_batch[0].clone()
        .into_iter()
        .map(|c| { c.to_string() })
        .collect());

    out
}
