import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import AppLink from '~/_components/appLink';
import Dropdown from '~/_components/buttons/dropdown';
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
			<React.Fragment>
				<div className="level is-mobile">
					<div className="level-left">
						<h4 className="title is-4">{this.props.account.name}</h4>
					</div>
					<div className="level-right">
						<div className="level-item">
							<Dropdown label="Actions">
								<div className="dropdown-content">
									<AppLink className="dropdown-item" to={"/money_maps/"+this.props.moneyMap.id+"/accounts/"+this.props.account.id+"/edit"}>Edit Account</AppLink>
								</div>
							</Dropdown>
						</div>
					</div>
				</div>
				<Panel name="Account Details" className="account-details">
					<StaticField label="Account Name" value={this.props.account.name} />
					<StaticField label="Created" value={this.props.account.created} />
					<StaticField label="Account Type" value={this.props.account.account_type} />
					<StaticField label="Balance" value="" />
				</Panel>
			</React.Fragment>
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
