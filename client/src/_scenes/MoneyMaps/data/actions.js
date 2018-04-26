import { globalConstants, moneyMapsConstants, alertConstants } from '~/_constants';
import { batchActions } from 'redux-batched-actions';
import MoneyMapsApi from './api';
import { history } from '~/_helpers/history';

/*
* getMoneyMaps
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
* createMoneyMap
*
* Create a money map
* @param {string} accountDetails
*/
const createMoneyMap = (moneyMap, redirectPath) => {
	return dispatch => {
		dispatch(request());

		MoneyMapsApi.createMoneyMap(moneyMap)
			.then(res => {
				if (res.status !== 'success')
					throw(res);
				dispatch(success(res.data));
				history.push(redirectPath);
			}).catch(err => {
				dispatch(failure(err.error));
			});
	};

	function request() { 
		return batchActions([
			{ type: moneyMapsConstants.CREATE_MONEY_MAP_REQUEST },
			{ type: alertConstants.CLEAR_ALERTS }
		]);
	}
	
	function success(user) {
		return batchActions([
			{ type: moneyMapsConstants.CREATE_MONEY_MAP_SUCCESS, user },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_SUCCESS, message: 'Money Map created successfully!'} }
		]);
	}

	function failure(user) {
		return batchActions([
			{ type: moneyMapsConstants.CREATE_MONEY_MAP_FAILURE, user },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Failed to create Money Map.'} }
		]);
	}
}

/*
* updateMoneyMap
*
* Update a money map
* @param {string} accountDetails
*/
const updateMoneyMap = (moneyMap, redirectPath) => {
	return dispatch => {
		dispatch(request());

		MoneyMapsApi.updateMoneyMap(moneyMap)
			.then(res => {
				if (res.status !== 'success')
					throw(res);
				dispatch(success(res.data));
				history.push(redirectPath);
			}).catch(err => {
				dispatch(failure(err.error));
			});
	};

	function request() { 
		return batchActions([
			{ type: moneyMapsConstants.UPDATE_MONEY_MAP_REQUEST },
			{ type: alertConstants.CLEAR_ALERTS }
		]);
	}
	
	function success(user) {
		return batchActions([
			{ type: moneyMapsConstants.UPDATE_MONEY_MAP_SUCCESS, user },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_SUCCESS, message: 'Money Map updated successfully!'} }
		]);
	}

	function failure(user) {
		return batchActions([
			{ type: moneyMapsConstants.UPDATE_MONEY_MAP_FAILURE, user },
			{ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Failed to update Money Map.'} }
		]);
	}
}

const cantFindMoneyMap = (redirectPath) => {
	return dispatch => {
		dispatch({ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Failed to find Money Map.'} });

		history.push(redirectPath);
	};

}


const moneyMapActions = {
	getMoneyMaps,
	createMoneyMap,
	updateMoneyMap,
	cantFindMoneyMap
}

export default moneyMapActions;