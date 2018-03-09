import { alertConstants } from '~/_constants';

const removeAlert = (id) => {
	return {
		type: alertConstants.REMOVE_ALERT,
		id: id
	};
}

const alertActions = {
	removeAlert
}

export default alertActions;