//! 添付ファイル(PDF)取得APIのデータモデル
//!
//! 技術基準適合証明等を受けた機器の検索Web-APIのリクエスト条件一覧(Ver.1.1.1)
//!
//! https://www.tele.soumu.go.jp/resource/j/giteki/webapi/gk_req_conditions.pdf

use serde::Serialize;

/// 添付ファイル(PDF)取得APIのリクエストパラメータ
#[derive(Serialize)]
pub struct RequestParameters {
    /// 添付ファイル取得キー
    ///
    /// 一覧詳細情報取得APIで取得した添付ファイル取得キーを指定。
    #[serde(rename = "AFK")]
    pub afk: String,

    /// 添付ファイル種別
    ///
    /// 添付ファイルの種別を指定。
    ///
    /// 1：外観写真等
    ///
    /// 2：特性試験の結果
    #[serde(rename = "AFT")]
    pub aft: Option<u8>,

    /// 取得形式
    ///
    /// 添付ファイル種別件数を元に対象添付ファイルの番号を指定。
    /// 添付ファイル番号を指定する場合、添付ファイル種別の指定が必要。
    #[serde(rename = "AFN")]
    pub afn: Option<u8>,
}

impl RequestParameters {
    pub fn new<S: Into<String>>(afk: S) -> RequestParameters {
        RequestParameters {
            afk: afk.into(),
            aft: None,
            afn: None,
        }
    }

    pub fn set_aft<T: Into<u8>>(&mut self, aft: T) {
        self.aft = Option::from(aft.into());
    }

    pub fn set_afn<T: Into<u8>>(&mut self, afn: T) {
        self.afn = Option::from(afn.into());
    }
}
