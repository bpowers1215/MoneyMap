import { userConstants } from '~/_constants';

const initialState = {
	auth: {
		email: '',
		id: '',
		firstName: '',
		lastName: '',
		token: ''
	}
}

const rootReducer = (state = initialState, action) => {
	
	switch (action.type) {
		case userConstants.LOGIN_SUCCESS:
		case userConstants.GET_ACCOUNT_SUCCESS: {
			let { email, id, first_name, last_name, token } = action.user;
			return {
				...state,
				auth: {
					email: email,
					id: id,
					firstName: first_name,
					lastName: last_name,
					token: token
				}
			}
		}
		default:
			return state;
	}
};

export default rootReducer;