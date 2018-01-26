import { userConstants } from '~/_constants';
import UsersApi from './api';

const login = (email, password) => {
	return dispatch => {
		dispatch(request({ email }));

		UsersApi.login(email, password)
			.then(
				res => {
					dispatch(success(res.data));
				},
				err => {
					dispatch(failure(err.error));
				}
			);
	};

	function request() { return { type: userConstants.LOGIN_REQUEST } }
	function success(user) { return { type: userConstants.LOGIN_SUCCESS, user } }
	function failure(error) { return { type: userConstants.LOGIN_FAILURE, error } }
}

const userActions = {
	login
}

export default userActions;