import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {}
};

const mapStateToProps = state => {
	return {}
}

class ConnectedAccountDetails extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	componentWillReceiveProps(nextProps){
	}
	render() {
		return (
			<h4 className="title is-4">{this.props.account.name}</h4>
		);
	}
}

const AccountDetails = connect(mapStateToProps, mapDispatchToProps)(ConnectedAccountDetails);

AccountDetails.defaultProps = {
	account:{}
}

AccountDetails.propTypes = {
	account: PropTypes.object.isRequired
}

export default AccountDetails;
