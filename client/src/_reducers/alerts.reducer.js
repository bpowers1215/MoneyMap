import { globalConstants } from '~/_constants';

const initialState = {
	alerts: []
}

const alertReducer = (state = initialState, action) => {
	
	switch (action.type) {
		case globalConstants.ADD_ALERT: {
			return {
				...state,
				alerts: [...state.alerts, action.alert]
			}
		}
		default:
			return state;
	}
};

export default alertReducer;