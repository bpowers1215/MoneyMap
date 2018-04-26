import { moneyMapsConstants } from '~/_constants';

const initialState = {
	editEnabled: false
}

const editMoneyMapFormReducer = (state = initialState, action) => {
	
	switch (action.type) {
		case moneyMapsConstants.MONEY_MAP_ENABLE_EDITABLE_FORM: {
			return {
				...state,
				editEnabled: true
			}
		}
		case moneyMapsConstants.UPDATE_MONEY_MAP_SUCCESS: {
			return {
				...state,
				editEnabled: false
			}
		}
		default:
			return state;
	}
};

export default editMoneyMapFormReducer;