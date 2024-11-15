#!/usr/bin/env python3

from uuid import uuid4
from web3 import Web3


class BlockChain:
    def __init__(self,address,account):
        self.address = address
        self.account = account

    def create_account(self, account):
        if account is None:
            return None
        else:
            account =  web3.eth.account.create()
            return account
    
    def currency_conversion(self, currency: int) -> int:
        if currency == currency.Web3.from_wei:
            return currency.Web3.to_wei
        else:
            return currency.Web3.from_wei

    def get_accounts(self, accounts 
