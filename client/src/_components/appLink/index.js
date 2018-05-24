import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import AppLinkActions from './actions';

/*
* AppLink
* Application Link - generates a anchor tag with onclick handler to 
* navigate the user to a new path. 
*
* By default, clears all alerts. This can be turned off using the clearAlerts bool property
*/

const mapDispatchToProps = dispatch => {
	return {
		navigate: (path, clearAlerts) => dispatch(AppLinkActions.navigate(path, clearAlerts))
	};
};

const mapStateToProps = state => {
	return {}
}

class ConnectedAppLink extends Component {
	constructor(props) {
		super(props);
		this.onClickHandler = this.onClickHandler.bind(this);
	}
	onClickHandler(event){
		event.preventDefault();
		this.props.navigate(this.props.to, this.props.clearAlerts);
	}
	render(){
		let {to, clearAlerts, children, navigate, ...rest} = this.props;
		return (
			<a
				{...rest}
				onClick={this.onClickHandler}
			>
				{this.props.children}
			</a>
		)
	}
}

const AppLink = connect(mapStateToProps, mapDispatchToProps)(ConnectedAppLink);

ConnectedAppLink.defaultProps = {
	clearAlerts: true
}

ConnectedAppLink.propTypes = {
	to: PropTypes.string.isRequired,
	clearAlerts: PropTypes.bool
}

export default AppLink;