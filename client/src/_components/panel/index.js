import React, { Component } from 'react'
import PropTypes from 'prop-types'
import FontAwesomeIcon from 'react-fontawesome';
import './styles.scss'

/*
* Panel component
*/
class Panel extends Component {
	constructor(props) {
		super(props)
		this.state = {
			show: props.displayDefault
		}

		this.toggleContent = this.toggleContent.bind(this);
	}
	toggleContent() {
		let state = { ...this.state, show: !this.state.show }
		this.setState(state);
	}
	render() {
		return (
			<div className={"card " +this.props.className}>
				<header className="card-header is-primary">
					<p className="card-header-title">
						{this.props.name}
					</p>
					{this.props.allowToggle && (
						<a className="card-header-icon" aria-label="more options" onClick={this.toggleContent}>
							{this.state.show && (
								<span className="icon">
									<FontAwesomeIcon name='angle-up'/>
								</span>
							)}
							{!this.state.show && (
								<span className="icon">
									<FontAwesomeIcon name='angle-down'/>
								</span>
							)}
						</a>
					)}
				</header>
				<div className={"card-content " + (this.state.show ? 'is-active' : '')}>
					<div className="content">
						{this.props.children}
					</div>
				</div>
			</div>
		)
	}
}

Panel.defaultProps = {
	className: '',
	id: '',
	displayDefault: false,
	allowToggle: true
}

Panel.propTypes = {
	name: PropTypes.string,
	className: PropTypes.string,
	id: PropTypes.string,
	displayDefault: PropTypes.bool,
	allowToggle: PropTypes.bool
}

export default Panel;