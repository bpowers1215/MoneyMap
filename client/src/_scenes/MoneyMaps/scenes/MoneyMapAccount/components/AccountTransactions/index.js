import React, { Component } from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import Panel from '~/_components/panel';
import './styles.scss';

const mapDispatchToProps = dispatch => {
	return {}
};

const mapStateToProps = state => {
	return {}
}

class ConnectedAccountTransactions extends Component {
	constructor(props) {
		super(props);
		this.state = {};
	}
	componentWillReceiveProps(nextProps){
	}
	render() {
		return (
			<React.Fragment>
				<Panel name="Account Transactions" className="account-details" displayDefault={true} allowToggle={false}>
				<table className="table">
					<thead>
						<tr>
							<th>Date</th>
							<th>Party</th>
							<th>Category</th>
							<th>Description</th>
							<th>Type</th>
							<th>Amount</th>
							<th>Balance</th>
						</tr>
					</thead>
					<tfoot>
						<tr>
							<th>Date</th>
							<th>Party</th>
							<th>Category</th>
							<th>Description</th>
							<th>Type</th>
							<th>Amount</th>
							<th>Balance</th>
						</tr>
					</tfoot>
					<tbody>
						<tr>
							<td>Jun 5</td>
							<td>Publix</td>
							<td>Groceries</td>
							<td>Done went shopping again</td>
							<td>Debit</td>
							<td>$100.00</td>
							<td>$350.00</td>
						</tr>
						<tr>
							<td>Jun 4</td>
							<td>Publix</td>
							<td>Groceries</td>
							<td>Done went shopping</td>
							<td>Debit</td>
							<td>$50.00</td>
							<td>$450.00</td>
						</tr>
					</tbody>
					</table>
				</Panel>
			</React.Fragment>
		);
	}
}

const AccountTransactions = connect(mapStateToProps, mapDispatchToProps)(ConnectedAccountTransactions);

AccountTransactions.defaultProps = {
	account:{}
}

AccountTransactions.propTypes = {
	account: PropTypes.object.isRequired
}

export default AccountTransactions;
