import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import AppLink from '~/_components/appLink';
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
	componentWillReceiveProps(nextProps){
	}
	getAccountLink(accountId){
		return "/money_maps/"+this.props.moneyMapId+"/accounts/"+accountId;
	}
	render() {
		let accounts = this.props.accounts;
		return (
			<table className="table is-fullwidth is-striped is-hoverable">
				<thead>
					<tr>
						<th>Account Name</th>
						<th>Account Type</th>
						<th>Balance</th>
					</tr>
				</thead>
				{accounts.length > 5 &&
					<tfoot>
						<tr>
							<th>Account Name</th>
							<th>Account Type</th>
							<th>Balance</th>
						</tr>
					</tfoot>
				}
				<tbody>
					{Object.keys(accounts).map((id) => 
						<tr key={id}>
							<td>
								<AppLink to={this.getAccountLink(id)}>
									{accounts[id].name}
								</AppLink>
							</td>
							<td>
								{accounts[id].account_type}
							</td>
							<td></td>
						</tr>
					)}
				</tbody>
			</table>
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
