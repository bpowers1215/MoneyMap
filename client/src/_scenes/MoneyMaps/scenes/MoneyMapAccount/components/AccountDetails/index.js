import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { StaticField } from '~/_components/form';
import Panel from '~/_components/panel';
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
			<Panel name="Account Details" className="account-details">
				<StaticField label="Account Name" value={this.props.account.name} />
				<StaticField label="Created" value={this.props.account.created} />
				<StaticField label="Account Type" value={this.props.account.account_type} />
				<StaticField label="Balance" value="" />
			</Panel>
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
