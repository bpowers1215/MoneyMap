import { globalConstants, userConstants } from '~/_constants';

const initialState = {
	editEnabled: false
}

const accountInfoReducer = (state = initialState, action) => {
	
	switch (action.type) {
		case globalConstants.ENABLE_EDITABLE_FORM: {
			return {
				...state,
				editEnabled: true
			}
		}
		case userConstants.UPDATE_ACCOUNT_SUCCESS: {
			return {
				...state,
				editEnabled: false
			}
		}
		default:
			return state;
	}
};

export default accountInfoReducer;