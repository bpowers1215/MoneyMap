import React from 'react';
import { Route } from 'react-router';
import { Redirect } from 'react-router-dom';
import store from '~/_helpers/store';
import { globalConstants, alertConstants } from '~/_constants';
import Auth from '~/_helpers/auth';

const PrivateRoute = ({ component: Component, ...rest }) => (
	<Route
		{...rest}
		render={ props => {
				if ( Auth.isAuthenticated() )
					return <Component {...props} />
				else {
					store.dispatch({ type: alertConstants.ADD_ALERT, alert: { className: globalConstants.STYLES.IS_DANGER, message: 'Sorry, you must log in to do that.'} });
					return <Redirect
						to={{
							pathname: "/login",
							state: { from: props.location }
						}}
					/>
				}
			}
		}
	/>
);

export default PrivateRoute;