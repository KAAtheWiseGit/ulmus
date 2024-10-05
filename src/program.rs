use std::sync::mpsc;

use crate::interface::Msg;

pub struct Program<M, T>
where
	M: crate::Model<CustomMsg = T>,
	T: Sized,
{
	model: M,
	reciever: mpsc::Receiver<Msg<T>>,
	sender: mpsc::Sender<Msg<T>>,
}
