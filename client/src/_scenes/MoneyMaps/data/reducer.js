import { moneyMapsConstants } from '~/_constants';

const initialState = {
	moneyMaps: []
}

const moneyMapsReducer = (state = initialState, action) => {
	
	switch (action.type) {
		case moneyMapsConstants.GET_MONEY_MAPS_SUCCESS: {
			let { moneyMaps } = action;
			return {
				...state,
				moneyMaps: moneyMaps
			}
		}
		default:
			return state;
	}
};

export default moneyMapsReducer;