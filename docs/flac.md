`flac`全称叫`Free Lossless Audio Codec`

`flac`的spec在 https://datatracker.ietf.org/doc/html/rfc9639

其他可能有用的链接 https://xiph.org/flac/documentation_tools.html

FLAC 之所以能够实现无损压缩，是因为音频信号中的样本往往与其相邻样本高度相关。与通常使用字典、游程编码或利用长期重复的通用压缩器不同，FLAC 仅在极短的时间内去除冗余，最多只回溯 32 个样本。

FLAC 格式提供的编码方法最适用于采样点以零为中心且具有符号表示的 PCM 音频信号。采样点以无符号表示的音频信号必须转换为符号表示，才能实现合理的压缩。FLAC 格式不适用于压缩非 PCM 音频。

flac使用的几种技术
- interchannel decorrelation.(通道间去相关)
- linear predictive coding (LPC) 线性预测编码
- Rice code

FLAC文件的编码过程遵循以下步骤。要解码 FLAC 文件，这些步骤则按相反的顺序执行
1. Blocking
2. Interchannel Decorrelation
3. Prediction
4. Residual Coding

FLAC 规定最小块大小为 16 个采样点（最后一个块除外），最大块大小为 65535 个采样点。

## 格式原则

所有编码和解码为 FLAC 格式的样本必须采用有符号表示 

FLAC 比特流中的一元编码采用零位到一位结尾的编码方式，例如，数字 5 的一元编码为 0b000001。这样可以防止帧同步码出现在一元编码的数字中

## 格式布局

音频数据由一个或多个音频帧组成。每个帧包含一个帧头，帧头包含同步码、帧信息（例如块大小、采样率和声道数）以及一个 8 位 CRC 校验码