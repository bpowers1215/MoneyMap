import { globalConstants, userConstants, alertConstants } from '~/_constants';
import { batchActions } from 'redux-batched-actions';
import { setCookie } from 'redux-cookie';
import UsersApi from './api';
import { history } from '~/_helpers/history';

/*
* login
* Authenticate a user
* On Success, set authentication cookie and redirect user
* On Error, display error messaging
* @param {string} email
* @param {string} password
*/
const login = (email, password) => {
	return dispatch => {
		dispatch(request({ email }));

		UsersApi.login(email, password)
			.then(res => {
					if (res.status !== 'success')
						throw(res);
					dispatch(success(res.data));
					dispatch(setAuthCookie(res.data));
					history.push('/account');
			}).catch(err => {
				dispatch(failure(err.error));
			});
	};

	function request() { 
		return batchActions([
			{ type: userConstants.LOGIN_REQUEST },
			{ type: alertConstants.CLEAR_ALERTS }
		]);
	}
	
	function success(user) {
		return batchActions([
			{ type: userConstants.LOGIN_SUCCESS, user },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_SUCCESS, message: 'Welcome!'} }
		]);
	}

	/**
	 * Because reduc-batched-actions does not pick up the 
	 * cookie middleware actions, we must dispatch it separately
	 * 
	 * @param {Object} user 
	 */
	function setAuthCookie(user) {
		let { token } = user;
		return setCookie(globalConstants.AUTH_TOKEN_COOKIE, token);
	}

	function failure(user) {
		return batchActions([
			{ type: userConstants.LOGIN_FAILURE, user },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Username or password did not match.'} }
		]);
	}
}

/*
* getAccount
* Get the account for the authenticated user 
*/

const getAccount = () => {
	return dispatch => {

		UsersApi.getAccount()
			.then(res => {
				if (res.status !== 'success')
					throw(res);
				dispatch(success(res.data));
			}).catch(err => {
				dispatch(failure(err.error));
			});
	};
	
	function success(user) {
		return batchActions([
			{ type: userConstants.GET_ACCOUNT_SUCCESS, user }
		]);
	}

	function failure(user) {
		return batchActions([
			{ type: userConstants.GET_ACCOUNT_FAILURE, user },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Unable to get account details.'} }
		]);
	}
}

const userActions = {
	login,
	getAccount
}

export default userActions;