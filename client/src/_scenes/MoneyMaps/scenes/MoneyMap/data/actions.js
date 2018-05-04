import { globalConstants, moneyMapsConstants, alertConstants } from '~/_constants';
import { batchActions } from 'redux-batched-actions';
import AccountsApi from './api';
import { history } from '~/_helpers/history';

/*
* getAccounts
*
* Get the accounts for a money map
*/
const getAccounts = () => {
	return dispatch => {

		AccountsApi.getAccounts()
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
			{ type: moneyMapsConstants.GET_ACCOUNTS_SUCCESS, moneyMaps }
		]);
	}

	function failure(error) {
		return batchActions([
			{ type: moneyMapsConstants.GET_ACCOUNTS_FAILURE, error },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Unable to get accounts.'} }
		]);
	}
}


const accountsActions = {
	getAccounts
}

export default accountsActions;