import { globalConstants, userConstants } from '~/_constants';
import { batchActions } from 'redux-batched-actions';
import UsersApi from './api';

const login = (email, password) => {
	return dispatch => {
		dispatch(request({ email }));

		UsersApi.login(email, password)
			.then(
				res => {
					dispatch(success(res.data));
				}
			).catch(err => {
				dispatch(failure(err.error));
			})
	};

	function request() { 
		return batchActions([
			{ type: userConstants.LOGIN_REQUEST },
			{ type: globalConstants.CLEAR_ALERTS }
		]);
	}
	function success(user) {
		return batchActions([
			{ type: userConstants.LOGIN_SUCCESS, user },
			{ type: globalConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.ALERT_SUCCESS, message: 'Welcome!'} }
		]);
	}
	function failure(user) { 
		return batchActions([
			{ type: userConstants.LOGIN_FAILURE, user },
			{ type: globalConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.ALERT_DANGER, message: 'Username or password did not match.'} }
		]);
	}
}

const userActions = {
	login
}

export default userActions;