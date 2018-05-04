import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import Icon from '~/_components/icon';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {}
};

const mapStateToProps = state => {
	return {}
}

class ConnectedAccountsList extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	render() {
		let accounts = this.props.accounts;
		return (
			<div className="">
				{Object.keys(accounts).map((id, index) => 
					<div>test</div>
				)}
			</div>
		);
	}
}

const AccountsList = connect(mapStateToProps, mapDispatchToProps)(ConnectedAccountsList);

AccountsList.defaultProps = {
	accounts:{}
}

AccountsList.propTypes = {
	accounts: PropTypes.object.isRequired
}

export default AccountsList;
