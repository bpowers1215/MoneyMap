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

	function request() { return { type: userConstants.LOGIN_REQUEST } }
	function success(user) { return { type: userConstants.LOGIN_SUCCESS, user } }
	function failure(user) { 
		return batchActions([
			{ type: userConstants.LOGIN_FAILURE, user },
			{ type: globalConstants.ADD_ALERT, alert: { text: 'ADD_ALERT MESSAGE'} }
		]);
	}
}

const userActions = {
	login
}

export default userActions;