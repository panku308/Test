import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://outlook247.com.au/rra/login.php')

WebUI.setText(findTestObject('New Folder/ViewInquiry/Page_Vip Recover/input_email'), 'pl250401@gmail.com')

WebUI.setText(findTestObject('New Folder/ViewInquiry/Page_Vip Recover/input_password'), 'test')

WebUI.click(findTestObject('New Folder/ViewInquiry/Page_Vip Recover/input_login'))

WebUI.delay(2)

WebUI.click(findTestObject('New Folder/Debug/Page_Vip Recover (2)/a_Support Centre'))

WebUI.click(findTestObject('New Folder/ViewInquiry/Page_Vip Recover/a_View Enquiries'))

x = WebUI.getUrl()

WebUI.verifyMatch(x, 'http://outlook247.com.au/rra/mytickets.php', false)

WebUI.closeBrowser()

