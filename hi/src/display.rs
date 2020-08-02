
const NV_VIBRANCE_VALUES: [u8; 51] = [0,1,3,4,5,6,8,9,10,11,13,14,15,16,18,19,20,21,23,24,25,26,28,29,30,32,33,34,35,37,38,39,40,42,43,44,45,
47,48,49,50,52,53,54,55,57,58,59,60,62,63];

#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Display(nvapi::Display);

impl From<nvapi::Display> for Display {
    fn from(d: nvapi::Display) -> Self {
        Self(d)
    }
}

impl std::ops::Deref for Display {
    type Target = nvapi::Display;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display {
    pub fn get_vibrance_data(&self) -> nvapi::Result<nvapi::DvcInfo> {
        self.0.dvc_read()
    }

    pub fn set_vibrance_data(&self, level: i32) -> nvapi::Result<nvapi::DvcInfo> {
        self.0.dvc_set(level)
    }

    pub fn get_vibrance(&self) -> nvapi::Result<u8> {
        let dvc = self.get_vibrance_data()?;
        let pos = NV_VIBRANCE_VALUES.iter().position(|v| *v as i32 == dvc.current_level).ok_or_else(|| nvapi::Status::SettingNotFound)?;
        Ok(pos as u8 + 50)
    }

    pub fn set_vibrance(&self, mut vibrance_percent: u8) -> nvapi::Result<u8> {
        if vibrance_percent < 50 {
            vibrance_percent = 50;
        } else if vibrance_percent > 100 {
            vibrance_percent = 100;
        }

        vibrance_percent -= 50;

        let _ = self.0.dvc_set(NV_VIBRANCE_VALUES[vibrance_percent as usize].into())?;
        self.get_vibrance()
    }
}
