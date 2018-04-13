import { combineReducers } from 'redux'
import users from './Users/reducer'
import moneyMaps from './MoneyMaps/reducer'

export default combineReducers({
	users,
	moneyMaps
})