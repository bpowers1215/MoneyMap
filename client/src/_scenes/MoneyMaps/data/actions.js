import { globalConstants, moneyMapsConstants, alertConstants } from '~/_constants';
import { batchActions } from 'redux-batched-actions';
import { setCookie } from 'redux-cookie';
import MoneyMapsApi from './api';
import { history } from '~/_helpers/history';

/*
* getgetMoneyMapsccount
*
* Get the users money maps
*/

const getMoneyMaps = () => {
	return dispatch => {

		MoneyMapsApi.getMoneyMaps()
			.then(res => {
				if (res.status !== 'success')
					throw(res);
				dispatch(success(res.data));
			}).catch(err => {
				dispatch(failure(err.error));
			});
	};
	
	function success(moneyMaps) {
		return batchActions([
			{ type: moneyMapsConstants.GET_MONEY_MAPS_SUCCESS, moneyMaps }
		]);
	}

	function failure(error) {
		return batchActions([
			{ type: moneyMapsConstants.GET_MONEY_MAPS_FAILURE, error },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Unable to get money maps.'} }
		]);
	}
}

/*
* updateAccount
*
* Update user account information
* @param {string} accountDetails
*/
// const updateAccount = (accountDetails) => {
// 	return dispatch => {
// 		dispatch(request());

// 		UsersApi.updateAccount(accountDetails)
// 			.then(res => {
// 					if (res.status !== 'success')
// 						throw(res);
// 					dispatch(success(res.data));
// 			}).catch(err => {
// 				dispatch(failure(err.error));
// 			});
// 	};

// 	function request() { 
// 		return batchActions([
// 			{ type: alertConstants.CLEAR_ALERTS }
// 		]);
// 	}
	
// 	function success(user) {
// 		return batchActions([
// 			{ type: userConstants.UPDATE_ACCOUNT_SUCCESS, user },
// 			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_SUCCESS, message: 'Account Updated'} }
// 		]);
// 	}

// 	function failure(user) {
// 		return batchActions([
// 			{ type: userConstants.UPDATE_ACCOUNT_FAILURE, user },
// 			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Failed to update account.'} }
// 		]);
// 	}
// }

const userActions = {
	getMoneyMaps
}

export default userActions;