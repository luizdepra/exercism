local BankAccount = {}

function BankAccount:new()
    local account = { _balance = 0, _is_open = true }
    return setmetatable(account, { __index = self })
end

function BankAccount:balance()
    return self._balance
end

function BankAccount:deposit(amount)
    assert(self._is_open, 'account should be open to make a deposit')
    assert(amount > 0, 'deposit amount should be positive and non-zero')

    self._balance = self._balance + amount
end

function BankAccount:withdraw(amount)
    assert(self._is_open, 'account should be open to make a withdraw')
    assert(amount > 0, 'withdraw amount should be positive and non-zero')
    assert(amount < self._balance, 'withdraw amount should be less or equal than the balance')

    self._balance = self._balance - amount
end

function BankAccount:close()
    self._is_open = false
end

return BankAccount
