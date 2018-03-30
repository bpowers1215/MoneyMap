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
		case userConstants.GET_ACCOUNT_SUCCESS:
		case userConstants.UPDATE_ACCOUNT_SUCCESS: {
			let { email, id, first_name, last_name, token } = action.user;
			return {
				...state,
				auth: {
					email: !!email ? email : state.auth.email,
					id: id,
					firstName: !!first_name ? first_name : state.auth.firstName,
					lastName: !!last_name ? last_name : state.auth.lastName,
					token: token
				}
			}
		}
		default:
			return state;
	}
};

export default rootReducer;