import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import AppLink from '~/_components/appLink';
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
	componentWillReceiveProps(nextProps){
	}
	getAccountLink(account){
		return "/money_maps/"+this.props.moneyMapId+"/account/"+account.id;
	}
	render() {
		let accounts = this.props.accounts;
		return (
			<table className="table is-fullwidth">
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
					{Object.keys(accounts).map((id, index) => 
						<tr key={id}>
							<td>
								<AppLink to={this.getAccountLink(accounts[index])}>
									{accounts[index].name}
								</AppLink>
							</td>
							<td>
								{accounts[index].account_type}
							</td>
						</tr>
					)}
				</tbody>
			</table>
		);
	}
}

const AccountsList = connect(mapStateToProps, mapDispatchToProps)(ConnectedAccountsList);

AccountsList.defaultProps = {
	accounts:[]
}

AccountsList.propTypes = {
	accounts: PropTypes.array.isRequired
}

export default AccountsList;
