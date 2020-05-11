//! 一覧詳細情報取得APIのデータモデル
//!
//! 技術基準適合証明等を受けた機器の検索Web-APIのリクエスト条件一覧(Ver.1.1.1)
//!
//! https://www.tele.soumu.go.jp/resource/j/giteki/webapi/gk_req_conditions.pdf

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

/// 一覧詳細情報取得APIのリクエストパラメータ
#[derive(Serialize)]
pub struct RequestParameters {
    /// スタートカウント
    ///
    /// 一覧の取得開始件数を指定。
    #[serde(rename = "SC")]
    pub sc: u32,

    /// 取得件数
    ///
    /// 1：10件
    ///
    /// 2：20件
    ///
    /// 3：30件
    ///
    /// 4：50件
    ///
    /// 5：100件
    ///
    /// 6：500件
    ///
    /// 7：1000件
    #[serde(rename = "DC")]
    pub dc: u8,

    /// 取得形式
    ///
    /// 1：CSV
    ///
    /// 2：JSON
    ///
    /// 3：XML
    #[serde(rename = "OF")]
    pub of: u8,

    /// 氏名又は名称
    ///
    /// 「技術基準適合証明又は工事設計認証を受けた者若しくは技術基準適合自己確認の届出業者の氏名又は名称」を指定。(部分一致検索)
    #[serde(rename = "NAM")]
    pub nam: Option<String>,

    /// 番号
    ///
    /// 「技術基準適合証明番号、工事設計認証番号又は届出番号」を指定。
    #[serde(rename = "NUM")]
    pub num: Option<String>,

    /// 型式又は名称
    ///
    /// 「機器の型式又は名称」を指定。(部分一致検索)
    #[serde(rename = "TN")]
    pub tn: Option<String>,

    /// 認証機関コード
    ///
    /// ※「機関コード」を参照。
    #[serde(rename = "OC")]
    pub oc: Option<String>,

    /// 年月日(開始)
    ///
    ///年月日(開始)、年月日(終了)を指定(YYYYMMDDで指定)
    ///
    ///※年月日(開始)、年月日(終了)を指定した場合の動作については、「年月日による検索方法について」を参照。
    #[serde(rename = "DS")]
    pub ds: Option<String>,

    /// 年月日(終了)
    ///
    ///年月日(開始)、年月日(終了)を指定(YYYYMMDDで指定)
    ///
    ///※年月日(開始)、年月日(終了)を指定した場合の動作については、「年月日による検索方法について」を参照。
    #[serde(rename = "DE")]
    pub de: Option<String>,

    /// 添付ファイル有
    ///
    /// 1：有のみ
    #[serde(rename = "AFP")]
    pub afp: Option<u8>,

    /// BODYSAR対応
    ///
    /// 1：対応
    #[serde(rename = "BS")]
    pub bs: Option<u8>,

    /// 特定無線設備の種別
    ///
    /// ※「特定無線設備の種別」を参照。
    #[serde(rename = "REC")]
    pub rec: Option<String>,

    /// 技術基準適合証明の種類
    ///
    /// ※「技術基準適合証明の種類」を参照。
    #[serde(rename = "TEC")]
    pub tec: Option<String>,

    /// 並び替えキー
    ///
    /// ※「並び替えキー」を参照。
    #[serde(rename = "SK")]
    pub sk: u8,

    /// 文字コード
    ///
    /// 1：UTF-8　※デフォルト
    ///
    /// 2：Shift_JIS
    #[serde(rename = "MC")]
    pub mc: u8,
}

impl RequestParameters {
    pub fn new() -> RequestParameters {
        RequestParameters {
            sc: 0,
            dc: 5,
            of: 2,
            nam: None,
            num: None,
            tn: None,
            oc: None,
            ds: None,
            de: None,
            afp: None,
            bs: None,
            rec: None,
            tec: None,
            sk: 1,
            mc: 1,
        }
    }

    pub fn set_sc<T: Into<u32>>(&mut self, sc: T) {
        self.sc = sc.into();
    }

    pub fn set_dc<T: Into<u8>>(&mut self, dc: T) {
        self.dc = dc.into();
    }

    pub fn set_nam<S: Into<String>>(&mut self, nam: S) {
        self.nam = Option::from(nam.into());
    }

    pub fn set_num<S: Into<String>>(&mut self, num: S) {
        self.num = Option::from(num.into());
    }

    pub fn set_tn<S: Into<String>>(&mut self, tn: S) {
        self.tn = Option::from(tn.into());
    }

    pub fn set_oc<S: Into<String>>(&mut self, oc: S) {
        self.oc = Option::from(oc.into());
    }

    pub fn set_ds<S: Into<String>>(&mut self, ds: S) {
        self.ds = Option::from(ds.into());
    }

    pub fn set_de<S: Into<String>>(&mut self, de: S) {
        self.de = Option::from(de.into());
    }

    pub fn set_afp<T: Into<u8>>(&mut self, afp: T) {
        self.afp = Option::from(afp.into());
    }

    pub fn set_bs<T: Into<u8>>(&mut self, bs: T) {
        self.bs = Option::from(bs.into());
    }

    pub fn set_rec<S: Into<String>>(&mut self, rec: S) {
        self.rec = Option::from(rec.into());
    }

    pub fn set_tec<S: Into<String>>(&mut self, tec: S) {
        self.tec = Option::from(tec.into());
    }

    pub fn set_sk<T: Into<u8>>(&mut self, sk: T) {
        self.sk = sk.into();
    }
}

/// 一覧詳細情報取得APIのレスポンス
#[derive(Deserialize)]
pub struct Response {
    #[serde(rename = "gitekiInformation")]
    pub giteki_information: GitekiInformation,
    #[serde(rename = "giteki", skip_serializing_if = "Vec::is_empty", default)]
    pub giteki: Vec<GitekiInfo>,
}

#[derive(Deserialize)]
pub struct GitekiInformation {
    #[serde(rename = "lastUpdateDate")]
    pub last_update_date: String,
    #[serde(
        rename = "totalCount",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub total_count: usize,
}

#[derive(Deserialize)]
pub struct GitekiInfo {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub no: usize,
    #[serde(rename = "techCode")]
    pub tech_code: String,
    pub number: String,
    pub date: String,
    pub name: String,
    #[serde(rename = "radioEquipmentCode")]
    pub radio_equipment_code: String,
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "elecWave")]
    pub elec_wave: String,
    #[serde(rename = "spuriousRules")]
    pub spurious_rules: String,
    #[serde(rename = "bodySar")]
    pub body_sar: String,
    pub note: String,
    #[serde(rename = "organName")]
    pub organ_name: String,
    #[serde(rename = "attachmentFileName")]
    pub attachment_file_name: String,
    #[serde(rename = "attachmentFileKey")]
    pub attachment_file_key: String,
    #[serde(rename = "attachmentFileCntForCd1")]
    pub attachment_file_cnt_for_cd_1: String,
    #[serde(rename = "attachmentFileCntForCd2")]
    pub attachment_file_cnt_for_cd_2: String,
}
