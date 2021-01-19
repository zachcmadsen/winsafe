use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::IdStr;
use crate::funcs::PostQuitMessage;
use crate::gui::dialog_base::{AfterCreate, DialogBase};
use crate::gui::events::MsgEvents;
use crate::gui::main_loop::run_loop;
use crate::gui::traits::Parent;
use crate::handles::{HINSTANCE, HWND};
use crate::msg::WmSetIcon;

#[derive(Clone)]
pub struct DialogMain(Arc<Obj>);

struct Obj { // actual fields of DialogMain
	base: DialogBase,
	icon_id: Option<i32>,
	accel_table_id: Option<i32>,
}

impl Parent for DialogMain {
	fn hwnd_ref(&self) -> &HWND {
		self.0.base.hwnd_ref()
	}

	fn events_ref(&self) -> &MsgEvents {
		self.0.base.events_ref()
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> WinResult<()> + 'static>)
	{
		self.0.base.add_child_to_be_created(func);
	}
}

impl DialogMain {
	pub fn new(
		dialog_id: i32,
		icon_id: Option<i32>,
		accel_table_id: Option<i32>) -> DialogMain
	{
		let dlg = Self(
			Arc::new(
				Obj {
					base: DialogBase::new(None, dialog_id, AfterCreate::Nothing),
					icon_id,
					accel_table_id,
				},
			),
		);
		dlg.default_message_handlers();
		dlg
	}

	pub fn run_main(&self, cmd_show: Option<co::SW>) -> WinResult<i32> {
		self.0.base.create_dialog_param()?; // may panic
		let hinst = self.0.base.parent_hinstance()?;

		let haccel = match self.0.accel_table_id {
			None => None,
			Some(id) => Some(hinst.LoadAccelerators(IdStr::Id(id))?),
		};

		self.set_icon_if_any(hinst)?;
		self.hwnd_ref().ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));

		run_loop(self.hwnd_ref(), haccel) // blocks until window is closed
	}

	fn default_message_handlers(&self) {
		self.events_ref().wm_close({
			let self2 = self.clone();
			move || {
				self2.hwnd_ref().DestroyWindow();
			}
		});

		self.events_ref().wm_nc_destroy(|| {
			PostQuitMessage(0);
		});
	}

	fn set_icon_if_any(&self, hinst: HINSTANCE) -> WinResult<()> {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		if let Some(id) = self.0.icon_id {
			self.hwnd_ref().SendMessage(
				WmSetIcon {
					hicon: hinst.LoadImageIcon(IdStr::Id(id), 16, 16, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::SMALL,
				},
			);

			self.hwnd_ref().SendMessage(
				WmSetIcon {
					hicon: hinst.LoadImageIcon(IdStr::Id(id), 32, 32, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::BIG,
				},
			);
		}

		Ok(())
	}
}
