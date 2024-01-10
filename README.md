# 百度lac rust示例

非官方的百度lac rust示例。

fork from https://github.com/dimusic/baidu-lac-rs

## 测试环境

- macOS v12.6
- paddle inference v2.3.2/v2.4.2/v2.5.2
- lac v2.0.0

## 使用说明

使用本示例需要做一些准备工作，下载paddle inference库和lac model，设置环境变量等。

经过测试，在x86_64芯片架构的macOS，使用paddle inference v2.3.2，v2.4.2，v2.5.2，配合lac v2.0.0可以正常运行。

paddle inference 下载地址，选择c推理库。

- V2.3.2: https://paddleinference.paddlepaddle.org.cn/master/user_guides/download_lib.html#mac
- v2.4.2: https://www.paddlepaddle.org.cn/inference/v2.4/guides/install/download_lib.html#mac
- v2.5.2: https://www.paddlepaddle.org.cn/inference/v2.5/guides/install/download_lib.html#mac



下载好paddle_inference_c.zip后，解压，主目录下会有paddle_inference_c_install_dir文件夹。

lac_model已经配置好，不需要手动下载和配置。如果需要切换model，下载相关model后，再修改src/dict_utils.rs中的run函数对应的路径即可。

lac下载地址：https://github.com/baidu/lac/releases/tag/v2.0.0

由于 baidu-lac-rs 是基于 [paddle-inference-rust-api](https://github.com/dimusic/paddle-inference-rust-api)，需要在macOS环境中需要设置`LIB_PADDLE_C_INSTALL_DIR`和`DYLD_FALLBACK_LIBRARY_PATH`环境变量，已经写在run.sh脚本中。

```
LIB_PADDLE_C_INSTALL_DIR=$PWD/paddle_inference_c_install_dir DYLD_FALLBACK_LIBRARY_PATH=$PWD/paddle_inference_c_install_dir/paddle/lib cargo run --example basic
```

最后执行`run.sh`脚本，出现下面结果表示执行成功。

```sh
[baidu-lac-rs]: Elapsed: 1.47ms
result [
    OutputItem {
        word: "LAC",
        tag: "nz",
    },
    OutputItem {
        word: "是",
        tag: "v",
    },
    OutputItem {
        word: "个",
        tag: "q",
    },
    OutputItem {
        word: "优秀",
        tag: "a",
    },
    OutputItem {
        word: "的",
        tag: "u",
    },
    OutputItem {
        word: "分词",
        tag: "n",
    },
    OutputItem {
        word: "工具",
        tag: "n",
    },
]
E0108 08:50:50.798673 154957312 analysis_config.cc:389] Please compile with MKLDNN first to use MKLDNN
[baidu-lac-rs]: Elapsed: 1.47ms
result [
    OutputItem {
        word: "百度",
        tag: "ORG",
    },
    OutputItem {
        word: "是",
        tag: "v",
    },
    OutputItem {
        word: "一家",
        tag: "m",
    },
    OutputItem {
        word: "高科技",
        tag: "n",
    },
    OutputItem {
        word: "公司",
        tag: "n",
    },
]
```
