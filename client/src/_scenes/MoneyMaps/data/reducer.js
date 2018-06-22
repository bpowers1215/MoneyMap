import { moneyMapsConstants } from '~/_constants';

const initialState = {
	moneyMaps: {}
}

const moneyMapsReducer = (state = initialState, action) => {
	
	switch (action.type) {
		case moneyMapsConstants.GET_MONEY_MAPS_SUCCESS: {
			let { moneyMaps } = action;
			let moneyMapsById = {};

			//map money maps by money map ID for easy access
			moneyMaps.forEach((moneyMap)=> {
				let accountsById = {};
				//map accounts by account ID for easy access
				moneyMap.accounts.forEach((account)=> {
					accountsById[account.id] = account;
				});

				moneyMap.accounts = accountsById;
				moneyMapsById[moneyMap.id] = moneyMap;
			});

			return {
				...state,
				moneyMaps: moneyMapsById
			}
		}
		default:
			return state;
	}
};

export default moneyMapsReducer;