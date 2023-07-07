#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, COMPTR, HANDLE, HRES};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{shell_IFileDialog, shell_IModalWindow, shell_IShellItem};
use crate::vt::IFileDialogVT;

/// [`IFileSaveDialog`](crate::IFileSaveDialog) virtual table.
#[repr(C)]
pub struct IFileSaveDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub SetSaveAsItem: fn(COMPTR, COMPTR) -> HRES,
	pub SetProperties: fn(COMPTR, COMPTR) -> HRES,
	pub SetCollectedProperties: fn(COMPTR, COMPTR, BOOL) -> HRES,
	pub GetProperties: fn(COMPTR, *mut COMPTR) -> HRES,
	pub ApplyProperties: fn(COMPTR, COMPTR, COMPTR, HANDLE, COMPTR) -> HRES,
}

com_interface! { IFileSaveDialog: "84bccd23-5fde-4cdb-aea4-af64b83d78ab";
	/// [`IFileSaveDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
	/// COM interface over [`IFileSaveDialogVT`](crate::vt::IFileSaveDialogVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// Saving a TXT file:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, HWND, IFileSaveDialog};
	///
	/// let hparent: HWND; // initialized somewhere
	/// # let hparent = HWND::NULL;
	///
	/// let file_save = CoCreateInstance::<IFileSaveDialog>(
	///     &co::CLSID::FileSaveDialog,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	///
	/// file_save.SetFileTypes(&[
	///     ("Text files", "*.txt"),
	///     ("All files", "*.*"),
	/// ])?;
	/// file_save.SetFileTypeIndex(1)?;
	/// file_save.SetDefaultExtension("txt")?;
	///
	/// if file_save.Show(&hparent)? {
	///     let chosen_file = file_save.GetResult()?
	///         .GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///     println!("{}", chosen_file);
	/// }
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_IModalWindow for IFileSaveDialog {}
impl shell_IFileDialog for IFileSaveDialog {}
impl shell_IFileSaveDialog for IFileSaveDialog {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileSaveDialog`](crate::IFileSaveDialog).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileSaveDialog: shell_IFileDialog {
	/// [`IFileSaveDialog::SetSaveAsItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
	/// method.
	fn SetSaveAsItem(&self, psi: &impl shell_IShellItem) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileSaveDialogVT>(self).SetSaveAsItem)(
					self.ptr(),
					psi.ptr(),
				)
			},
		)
	}
}
